#[macro_use] extern crate rocket; 
use panicvtt_engine::{self, entities::entity::Entity};

use rocket::{form::Form, response::Redirect};
use rocket_dyn_templates::{Template, context};

//#[get("/")]
fn old_index() -> String {
    let mut data = String::new();
    
    data.push_str(&format!("PanicVTT version {}\n", panicvtt_engine::version()));

    let entity = panicvtt_engine::entities::entity::EntityBase::from_str("Sam Paniccia");
    data.push_str(&format!("By {} (UUID {})\n", entity.get_name(), entity.get_uuid())); 

    data
}

#[derive(FromForm)]
struct Command<'r> {
    command: &'r str
}

static mut COMMANDS: Vec<String> = Vec::new(); 

#[get("/")]
fn index() -> Template { 
    Template::render("index", context! { 
        items: unsafe { COMMANDS.clone() }
     })
}

#[post("/", data = "<form_data>")]
fn add_command(form_data: Option<Form<Command<'_>>>) -> Redirect {
    match form_data {
        Some(f) => {
            println!("Got {}", f.command); 
            unsafe {
                COMMANDS.push(String::from(f.command));
            }
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
    .mount("/", routes![index, add_command])
    .attach(Template::fairing())
}
