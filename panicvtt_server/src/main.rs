#[macro_use] extern crate rocket; 

use std::{fs::File, io::Read, sync::Mutex};
use parse_command::ParseError;
use rocket_dyn_templates::Template;
use panic_state::PanicState;
use routes::parse_command;
use serde::{Serialize, Deserialize};

mod models;
mod routes;
mod parse_command;
mod panic_state; 

#[derive(Serialize, Deserialize)]
struct InitialState {
    pub(self) commands: Vec<String>,
}
 

fn define_initial_state(state_path: Option<&str>) -> (Vec<String>, PanicState) {
    let mut commands: Vec<String> = Vec::new();
    let mut state = PanicState::new(panicvtt_engine::initialize());
    
    if let Some(path) = state_path {
        // Open and read JSON file 
        let mut file = File::open(path).expect(format!("Failed to open initial state file {}!", path).as_str());
        
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).expect("Failed to read initial state file!");

        // Deserialize 
        match serde_json::from_str::<InitialState>(&json_str) {
            Ok(initial_state) => { 
                // Attempt to parse each command
                match initial_state.commands.iter().map(|s| {
                    parse_command(s, &mut state)
                }).collect::<Result<Vec<_>, ParseError>>() {
                    Ok(mut v) => {
                        // Append the successful commands onto the command list 
                        commands.append(&mut v);
                        println!("DIAG: Applied {} init commands", commands.len());
                    }, 
                    Err(e) => {
                        // Something failed to parse. Reject the entire file
                        eprintln!("Failed to parse initial state command: {}!", e.faulty_token);
                    }
                }
            }, 
            Err(e) => {
                eprintln!("Failed to deserialize initial state file: {}!", e);
                return (commands, state);
            }
        }
    } else {
        println!("DIAG: No initial state file supplied");
    }

    (commands, state)
}

#[launch]
fn rocket() -> _ {
    let init_file = option_env!("PANICVTT_SERVER_INIT_FILE"); 
    let (commands, state) = define_initial_state(init_file);

    rocket::build()
    .manage(models::CommandList { commands: Mutex::new(commands) })
    .manage(Mutex::new(state)) 
    .mount("/", routes![routes::index, 
        routes::add_command, 
        routes::connect,
        routes::login_get,
        routes::login_post, 
        routes::disconnect, 
        routes::vtt])
    .attach(Template::fairing())
}
