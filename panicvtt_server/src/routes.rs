use panicvtt_engine;

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
    // !TODO
    Redirect::to("/vtt") 
}

#[post("/disconnect")]
pub async fn disconnect() -> Redirect {
    // !TODO 
    Redirect::to("/")
}
