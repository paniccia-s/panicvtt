use std::collections::HashMap;

use panicvtt_engine::{engine::Engine, entities::entityview::EntityView};

pub(super) struct PanicState {
    pub(super) engine: Engine,
    pub(super) entities: HashMap<String, EntityView>
}

impl PanicState {
    pub(super) fn new(engine: Engine) -> Self {
        Self {
            engine, 
            entities: HashMap::new()
        }
    }
}

