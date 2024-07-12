#[macro_use] extern crate rocket; 

use std::{env::Args, fs::File, io::Read, path::Path, sync::Mutex};
use parse_error::ParseError;
use rocket_dyn_templates::Template;
use panic_state::PanicState;
use routes::parse_command;
use serde::{Serialize, Deserialize};

mod models;
mod routes;
mod parse_command;
mod parse_error;
mod panic_state; 

#[derive(Serialize, Deserialize)]
struct InitialState {
    pub(self) commands: Vec<String>,
}
 
struct PanicArgs {
    asset_root: String,
}

impl PanicArgs {

    fn new(args: Args) -> Result<PanicArgs, String> {
        let args: Vec<String> = args.collect();
        if args.len() != 2 {
            return Err(String::from("Usage: panicvtt_server <asset_root>"));
        }

        Ok(PanicArgs {
            asset_root: args.get(1).unwrap().clone()
        })
    }
}


fn define_initial_state(state_path: Option<&str>, state: &mut PanicState) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    
    if let Some(path) = state_path {
        // Open and read JSON file 
        let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open initial state file {}!", path));
        
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).expect("Failed to read initial state file!");

        // Deserialize 
        match serde_json::from_str::<InitialState>(&json_str) {
            Ok(initial_state) => { 
                // Attempt to parse each command
                match initial_state.commands.iter().map(|s| {
                    parse_command(s, state)
                }).collect::<Result<Vec<_>, ParseError>>() {
                    Ok(mut v) => {
                        // Append the successful commands onto the command list 
                        commands.append(&mut v);
                        println!("DIAG: Applied {} init commands", commands.len());
                    }, 
                    Err(e) => {
                        // Something failed to parse. Reject the entire file
                        eprintln!("Failed to parse initial state command: {}!", e);
                    }
                }
            }, 
            Err(e) => {
                eprintln!("Failed to deserialize initial state file: {}!", e);
                return commands;
            }
        }
    } else {
        println!("DIAG: No initial state file supplied");
    }

    commands
}


#[main]
async fn main() -> Result<(), rocket::Error> {
    let Ok(args) = PanicArgs::new(std::env::args()) else {
        eprintln!("Usage: panicvtt_server <asset_root>");
        return Err(rocket::Error::from(rocket::error::ErrorKind::Io(
            std::io::Error::from(std::io::ErrorKind::InvalidInput)
        )));
    };

    let mut state = PanicState::new(panicvtt_engine::initialize(Path::new(&args.asset_root)));

    let init_file = option_env!("PANICVTT_SERVER_INIT_FILE"); 
    let commands = define_initial_state(init_file, &mut state);

    let _r = rocket::build()
        .manage(models::CommandList { commands: Mutex::new(commands) })
        .manage(Mutex::new(state)) 
        .mount("/", routes![routes::index, 
            routes::add_command, 
            routes::connect,
            routes::login_get,
            routes::login_post, 
            routes::disconnect, 
            routes::vtt, 
            routes::load_campaign_get,
            routes::load_campaign_put])
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())        
}

