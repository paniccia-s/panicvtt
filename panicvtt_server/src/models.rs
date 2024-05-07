use std::sync::Mutex;

#[derive(FromForm)]
pub struct Command<'r> {
    pub command: &'r str
}

pub struct CommandList {
    pub commands: Mutex<Vec<String>>
}
