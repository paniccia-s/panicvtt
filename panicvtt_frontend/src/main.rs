#[macro_use] extern crate rocket; 
use panicvtt_engine::{self, entities::entity::Entity};

#[get("/")]
fn index() -> String {
    let mut data = String::new();
    
    data.push_str(&format!("PanicVTT version {}\n", panicvtt_engine::version()));

    let entity = panicvtt_engine::entities::entity::EntityBase::from_str("Sam Paniccia");
    data.push_str(&format!("By {} (UUID {})\n", entity.get_name(), entity.get_uuid())); 

    data
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
