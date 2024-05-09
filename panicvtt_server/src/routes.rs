use std::sync::Mutex;

use panicvtt_engine;

use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::{Template, context};

use crate::{panic_state::PanicState, parse_command::{command_delete_entity, command_new_entity, ParseError}};

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
    Redirect::to("/vtt")
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
            format!("Failed to parse command: \"{}\" is an invalid token!", e.faulty_token)
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


const COMMAND_NEW_ENTITY: &str      = "new_entity";
const COMMAND_DELETE_ENTITY: &str   = "delete_entity"; 

fn parse_command(command: &str, state: &mut PanicState) -> Result<String, ParseError> {
    // Tokenize by whitespace
    let tokens: Vec<&str> = command.split_whitespace().collect();

    // Parse the tokens 
    return match tokens.first() {
        Some(cmd) => {
            match *cmd {
                COMMAND_NEW_ENTITY => {
                    command_new_entity(&tokens, state)
                }, 
                COMMAND_DELETE_ENTITY => {
                    command_delete_entity(&tokens, state)
                }, 
                _ => {
                    // Invalid token! 
                    Err(ParseError::new(*cmd, &tokens))
                }
            } 
        }, 
        None => {
            // We got nothing 
            Err(ParseError::new("", &tokens))
        }
    } 
}


