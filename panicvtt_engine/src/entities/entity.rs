use std::fmt::Display;

use uuid::Uuid;

/// An Entity is an agent within the engine that is able to be unique identified and interacted with. 
pub struct Entity {
    uuid: Uuid,
    name: String
}

impl Entity {
    pub fn new(name: String) -> Entity {
        Entity {
            uuid: Uuid::now_v7(),
            name
        }
    }
    

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_entity_data(f, &self.uuid, &self.name)
    }
}

pub(super) fn format_entity_data(f: &mut std::fmt::Formatter<'_>, uuid: &Uuid, name: &String) -> std::fmt::Result {
    let uuid_str = uuid.as_u128().to_string();
    write!(f, "Entity {} (uuid ...{})", name, &uuid_str[uuid_str.len() - 6..])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_new() {
        let name_raw = "David Gilmour";
        let name = String::from(name_raw);
        let entity = Entity::new(name);
        assert_eq!(entity.get_name(), name_raw);
    }

    #[test]
    fn entity_from() {
        let name_raw = "Rick Wright";
        let entity = Entity::_from_str(name_raw);
        assert_eq!(entity.get_name(), name_raw);
    }
}
