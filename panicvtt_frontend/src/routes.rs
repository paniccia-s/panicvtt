use std::net::Ipv4Addr;

use panicvtt_engine;

use panicvtt_net::panicnet;
use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::{Template, context};

use super::models::{Command, CommandList};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        version: panicvtt_engine::version(),
    })
}

#[get("/vtt")]
pub fn vtt(command_list: &State<CommandList>) -> Template { 
    let lock = command_list.commands.lock().expect("index");
    Template::render("vtt", context! { 
        items: lock.clone(), 
        version: panicvtt_engine::version(),
     })
}

#[post("/vtt", data = "<form_data>")]
pub fn add_command(form_data: Option<Form<Command<'_>>>, command_list: &State<CommandList>) -> Redirect {
    match form_data {
        Some(f) => {
            println!("Got {}", f.command); 
            let mut lock = command_list.commands.lock().expect("Lock shared data");
            lock.push(String::from(f.command));
        },
        None => {
            println!("??");
        }
    }
    
    Redirect::to("/vtt")
}

#[post("/connect")]
pub async fn connect() -> Redirect {
    let pn = panicnet::PanicNetClient::new(Ipv4Addr::LOCALHOST, 21918).await;
    match pn {
        Ok(pn) => {
            Redirect::to("/vtt")
        }, 
        Err(e) => { 
            Redirect::to("/")
        }
    }
}

#[post("/disconnect")]
pub async fn disconnect() -> Redirect {
    // !TODO 
    Redirect::to("/")
}
