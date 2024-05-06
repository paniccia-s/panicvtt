#[macro_use] extern crate rocket; 
use std::sync::Mutex;

use panicvtt_engine;

use rocket::{form::Form, response::Redirect, State};
use rocket_dyn_templates::{Template, context};

#[derive(FromForm)]
struct Command<'r> {
    command: &'r str
}

struct CommandList {
    commands: Mutex<Vec<String>>
}

#[get("/")]
fn index(command_list: &State<CommandList>) -> Template { 
    let lock = command_list.commands.lock().expect("index");
    Template::render("index", context! { 
        items: lock.clone(), 
        version: panicvtt_engine::version(),
     })
}

#[post("/", data = "<form_data>")]
fn add_command(form_data: Option<Form<Command<'_>>>, command_list: &State<CommandList>) -> Redirect {
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
    Redirect::to("/")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(CommandList { commands: Mutex::new(Vec::new()) })
    .mount("/", routes![index, add_command])
    .attach(Template::fairing())
}
