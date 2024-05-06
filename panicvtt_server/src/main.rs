use panicvtt_engine;

mod version;

pub fn version() -> &'static str {
    version::VERSION
}

fn main() {
    println!("PanicVTT Server version {}: Engine version {}", version(), panicvtt_engine::version());
}
