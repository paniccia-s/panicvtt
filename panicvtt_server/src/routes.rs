use std::sync::Mutex;

use panicvtt_engine::assets::load_asset_result::LoadAssetResult;
use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::{Template, context};

//use crate::{panic_state::PanicState, parse_command::{command_delete_entity, command_get_entity_abilities, command_get_entity_ability, command_list_entities, command_new_entity}, parse_error::ParseError};

use crate::{panic_state::PanicState, parse_error::ParseError};

use super::models::{Command, CommandList};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        subtitle: "- By GMs, For GMs.",
        version: panicvtt_engine::version(),
    })
}

#[post("/connect")]
pub fn connect() -> Redirect { 
    // !TODO
    Redirect::to("/login") 
}

#[get("/login")]
pub fn login_get() -> Template {
    Template::render("login", context! {
        subtitle: "- Log In", 
        version: panicvtt_engine::version(),
    })
}

#[derive(FromForm)]
pub struct Login {
    username: String, 
    password: String
}

#[post("/login", data = "<form_data>")]
pub fn login_post(form_data: Form<Login>) -> Redirect {
    // !TODO 
    println!("Login pair: {} {}", form_data.username, form_data.password);
    Redirect::to("/load_campaign")
}

#[get("/load_campaign")]
pub fn load_campaign_get(state: &State<Mutex<PanicState>>) -> Template {
    let lock = state.lock().unwrap();

    // Get loaded campaign information from the engine 
    let campaigns = lock.engine.get_campaign_descriptions();
    Template::render("load_campaign", context! {
        subtitle: "- Select Campaign!",
        campaigns: campaigns,
        version: panicvtt_engine::version(),
    })
}

#[derive(FromForm)]
pub struct LoadCampaign {
    uuid: String
}

#[post("/load_campaign", data = "<form_data>")]
pub fn load_campaign_put(form_data: Form<LoadCampaign>, state: &State<Mutex<PanicState>>) -> Redirect {
    println!("\t{}", form_data.uuid);
    
    match u128::from_str_radix(&form_data.uuid, 16) {
        Ok(u) => {
            let mut lock = state.lock().unwrap();
            let l = lock.engine.load_campaign(u);
            
            match l {
                LoadAssetResult::Ok { asset } => {
                    println!("Loaded campaign with name {}!", asset.get_name());
                    Redirect::to("/load_campaign")
                },
                LoadAssetResult::UuidNotFoundError => todo!(),
                LoadAssetResult::IoError { e } => todo!(),
                LoadAssetResult::NoCampaignFound => todo!(),
                LoadAssetResult::UuidDuplicateError { e } => todo!(),
            }
        }, 
        Err(e) => {
            // Uh oh 
            eprintln!("{}", e);
            Redirect::to("/load_campaign")
        }
    }
}

#[get("/vtt")]
pub fn vtt(command_list: &State<CommandList>) -> Template { 
    let lock = command_list.commands.lock().expect("index");
    Template::render("vtt", context! { 
        subtitle: "- By GMs, For GMs.",
        items: lock.clone(), 
        version: panicvtt_engine::version(),
     })
}

#[post("/vtt", data = "<form_data>")]
pub fn add_command(form_data: Form<Command<'_>>, command_list: &State<CommandList>, state: &State<Mutex<PanicState>>) -> Redirect {
    // Parse the command data 
    let mut lock = state.lock().unwrap();
    
    let message = match parse_command(form_data.command, &mut lock) {
        Ok(message) => message, 
        Err(e) => {
            format!("Failed to parse command: {}!", e)
        }
    }; 
                
    let mut command_lock = command_list.commands.lock().expect("Lock shared data");
    command_lock.push(message);
    
    // Refresh the page 
    Redirect::to("/vtt")
}

#[post("/disconnect")]
pub fn disconnect() -> Redirect {
    // !TODO 
    Redirect::to("/")
}


const _COMMAND_NEW_ENTITY:           &str = "new_entity";
const _COMMAND_DELETE_ENTITY:        &str = "delete_entity"; 
const _COMMAND_LIST_ENTITIES:        &str = "list_entities";
const _COMMAND_GET_ENTITY_ABILITY:   &str = "get_entity_ability";
const _COMMAND_GET_ENTITY_ABILITIES: &str = "get_entity_abilities";

pub(super) fn parse_command(_command: &str, _state: &mut PanicState) -> Result<String, ParseError> {
    todo!()
    // // Tokenize by whitespace
    // let tokens: Vec<&str> = command.split_whitespace().collect();

    // // Parse the tokens 
    // return match tokens.first() {
    //     Some(cmd) => {
    //         match *cmd {
    //             COMMAND_NEW_ENTITY              => command_new_entity(&tokens, state), 
    //             COMMAND_DELETE_ENTITY           => command_delete_entity(&tokens, state), 
    //             COMMAND_LIST_ENTITIES           => command_list_entities(&tokens, state),
    //             COMMAND_GET_ENTITY_ABILITY      => command_get_entity_ability(&tokens, state),
    //             COMMAND_GET_ENTITY_ABILITIES    => command_get_entity_abilities(&tokens, state),
    //             _ => {
    //                 // Invalid token! 
    //                 Err(ParseError::from_syntax_error(&tokens, cmd))
    //             }
    //         } 
    //     }, 
    //     None => {
    //         // We got nothing 
    //         Err(ParseError::from_wrong_num_args(&tokens, 0, 1))
    //     }
    // } 
}
