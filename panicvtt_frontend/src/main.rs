#[macro_use] extern crate rocket; 
use panicvtt_engine;

#[get("/")]
fn index() -> String {
    format!("PanicVTT version {}", panicvtt_engine::version())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
