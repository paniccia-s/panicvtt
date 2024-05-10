use std::collections::HashMap;

use panicvtt_engine::engine::Engine;

pub(super) struct PanicState {
    pub(super) engine: Engine,
    pub(super) entities: HashMap<String, u128>
}

impl PanicState {
    pub(super) fn new(engine: Engine) -> Self {
        Self {
            engine, 
            entities: HashMap::new()
        }
    }
}

