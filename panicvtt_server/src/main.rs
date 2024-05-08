#[macro_use] extern crate rocket; 

use std::sync::Mutex;
use rocket_dyn_templates::Template;

mod models;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(models::CommandList { commands: Mutex::new(Vec::new()) })
    .manage(Mutex::new(panicvtt_engine::initialize()))
    .mount("/", routes![routes::index, 
        routes::add_command, 
        routes::connect,
        routes::login_get,
        routes::login_post, 
        routes::disconnect, 
        routes::vtt])
    .attach(Template::fairing())
}
