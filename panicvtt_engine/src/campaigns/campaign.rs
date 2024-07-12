use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assets::asset::Asset;

use super::scene::Scene;

#[derive(Serialize, Deserialize, Debug)]
pub struct Campaign {
    #[serde(with = "uuid::serde::simple")]
    uuid: Uuid, 
    name: String, 
    description: String, 
    scenes: HashMap<u128, Scene>, 
    active_scene: Option<u128>, 
}

impl Campaign {
    pub fn new(name: String, description: String) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name, 
            description,
            scenes: HashMap::new(),
            active_scene: None
        }
    }

    pub fn with_no_description(name: String) -> Self {
        Self::new(name, String::from("A D&D Adventure!"))
    }


    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn create_scene(&mut self, scene_name: String) -> Option<&Scene> {
        let scene = Scene::new(scene_name);
        let uuid = scene.get_uuid();
        self.scenes.insert(uuid, scene);

        self.scenes.get(&uuid)
    }

    pub fn add_scene(&mut self, scene: Scene) {
        self.scenes.insert(scene.get_uuid(), scene);
    }

    pub fn get_scene(&self, id: u128) -> Option<&Scene> {
        self.scenes.get(&id)
    }
}
 
impl Asset for Campaign {
    fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }
    
    fn get_owning_campaign(&self) -> Option<u128> {
        // Campaigns are not owned 
        None
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{assets::asset::Asset, campaigns::scene::Scene};

    use super::Campaign;

    #[test]
    pub fn add_some_scenes() {
        let mut c = Campaign::new(String::from("The Gang Plays D&D"), String::from("A Roleplaying Adventure by Mantis Toboggan, M.D."));
        assert_eq!(c.get_name(), "The Gang Plays D&D");
        assert_eq!(c.get_description(), "A Roleplaying Adventure by Mantis Toboggan, M.D.");

        let s1 = Scene::new(String::from("scene 1"));
        let s3 = Scene::new(String::from("scene 3"));
        let id1 = s1.get_uuid();
        let id3 = s3.get_uuid();

        c.add_scene(s1);
        let id2 = c.create_scene(String::from("scene 2")).unwrap().get_uuid();
        c.add_scene(s3);
        let id4 = c.create_scene(String::from("scene 4")).unwrap().get_uuid();

        let s1 = c.get_scene(id1).unwrap();
        let s2 = c.get_scene(id2).unwrap();
        let s3 = c.get_scene(id3).unwrap();
        let s4 = c.get_scene(id4).unwrap();

        assert_eq!(s1.get_name(), "scene 1");
        assert_eq!(s2.get_name(), "scene 2");
        assert_eq!(s3.get_name(), "scene 3");
        assert_eq!(s4.get_name(), "scene 4");

        // Try to get some bad scenes 
        assert!(c.get_scene(55u128).is_none());
        assert!(c.get_scene(u128::MAX).is_none());
    }
}