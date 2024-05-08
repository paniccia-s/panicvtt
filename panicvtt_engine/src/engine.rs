use crate::entities::entity::Entity;



pub struct Engine {
    entities: Vec<Entity>,    
}

impl Engine {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self, name: &str) {
        let entity = Entity::new(String::from(name));
        self.entities.push(entity);

        println!("{} entities: [{}]", self.entities.len(), self.entities.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", "));
    }
}