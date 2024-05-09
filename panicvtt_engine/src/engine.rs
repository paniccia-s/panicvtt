use crate::entities::{entity::Entity, entityview::EntityView};



pub struct Engine {
    entities: Vec<Entity>,    
}

impl Engine {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self, name: &str) -> EntityView {
        let entity = Entity::new(String::from(name));
        let view = entity.to_view();
        self.entities.push(entity);

        view
    }
}