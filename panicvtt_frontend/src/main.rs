#[macro_use] extern crate rocket; 

use std::sync::Mutex;
use rocket_dyn_templates::Template;

mod routes;
mod models;

#[launch]
fn rocket() -> _ {
    rocket::build()
    .manage(models::CommandList { commands: Mutex::new(Vec::new()) })
    .mount("/", routes![routes::index, routes::add_command])
    .attach(Template::fairing())
}
