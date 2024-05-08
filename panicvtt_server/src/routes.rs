use std::{fmt::Display, sync::Mutex};

use panicvtt_engine::{self, engine::Engine};

use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::{Template, context};

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
pub fn add_command(form_data: Form<Command<'_>>, command_list: &State<CommandList>, engine: &State<Mutex<Engine>>) -> Redirect {
    // Parse the command data 
    let mut lock = engine.lock().unwrap();
    if let Err(e) = parse_command(form_data.command, &mut lock) {
        eprintln!("Failed to parse command: \"{}\" is an invalid token!", e.faulty_token);
    }
    
    // Add the command to the render list
    let mut lock = command_list.commands.lock().expect("Lock shared data");
    lock.push(String::from(form_data.command));
    
    // Refresh the page 
    Redirect::to("/vtt")
}

#[post("/disconnect")]
pub fn disconnect() -> Redirect {
    // !TODO 
    Redirect::to("/")
}


#[derive(Debug)]
struct ParseError {
    faulty_token: String, 
    all_tokens: Vec<String>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Failed to parse token {} (all tokens: [", self.faulty_token).as_str())?;
        self.all_tokens.iter().map(|t| f.write_str(format!("{} ", t).as_str())).collect()
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    fn new(faulty_token: &str, all_tokens: &Vec<&str>) -> Self {
        Self {
            faulty_token: String::from(faulty_token),
            all_tokens: all_tokens.iter().map(|s| { String::from(*s) }).collect()
        }
    }
}


const COMMAND_NEW_ENTITY: &str = "new_entity"; 

fn parse_command(command: &str, engine: &mut Engine) -> Result<(), ParseError> {
    // Tokenize by whitespace
    let tokens: Vec<&str> = command.split_whitespace().collect();

    // Parse the tokens 
    return match tokens.first() {
        Some(cmd) => {
            match *cmd {
                COMMAND_NEW_ENTITY => {
                    command_new_entity(&tokens, engine)
                }
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

/// Parameters: <entity_name>
fn command_new_entity(tokens: &Vec<&str>, engine: &mut Engine) -> Result<(), ParseError> {
    // Make sure everything is formatted correctly 
    if tokens.len() != 2 {
        return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    }

    if let Some(name) = tokens.get(1) {
        engine.create_entity(*name);
    } else {
        return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    }

    Ok(())
} 


