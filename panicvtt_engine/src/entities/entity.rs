use uuid::Uuid;

/// An Entity is an agent within the engine that is able to be unique identified and interacted with.
pub trait Entity {
    fn get_uuid(&self) -> u128;
}

pub struct EntityBase {
    uuid: Uuid,
    name: String
}

impl EntityBase {
    pub fn new(name: String) -> EntityBase {
        EntityBase {
            uuid: Uuid::now_v7(),
            name
        }
    }

    pub fn from_str(name: &'static str) -> EntityBase {
        Self::new(String::from(name))
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Entity for EntityBase {
    fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entitybase_new() {
        let name_raw = "David Gilmour";
        let name = String::from(name_raw);
        let entity = EntityBase::new(name);
        assert_eq!(entity.get_name(), name_raw);
    }

    #[test]
    fn entitybase_from() {
        let name_raw = "Rick Wright";
        let entity = EntityBase::from_str(name_raw);
        assert_eq!(entity.get_name(), name_raw);
    }
}
