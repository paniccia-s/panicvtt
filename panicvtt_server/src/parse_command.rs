use std::fmt::Display;

use crate::panic_state::PanicState;

#[derive(Debug)]
pub(super) struct ParseError {
    pub(super) faulty_token: String, 
    all_tokens: Vec<String>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Failed to parse token {} (all tokens: [", self.faulty_token).as_str())?;
        self.all_tokens.iter().map(|t| f.write_str(format!("{} ", t).as_str())).collect()
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    pub(super) fn new(faulty_token: &str, all_tokens: &Vec<&str>) -> Self {
        Self {
            faulty_token: String::from(faulty_token),
            all_tokens: all_tokens.iter().map(|s| { String::from(*s) }).collect()
        }
    }
}

/// Parameters: <entity_name>
pub(super) fn command_new_entity(tokens: &Vec<&str>, state: &mut PanicState) -> Result<String, ParseError> {
    // Make sure everything is formatted correctly 
    if tokens.len() != 2 {
        return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    }

    return if let Some(name) = tokens.get(1) {
        // !TODO do not allow name duplicates until we can resolve them through the webpage
        if state.entities.contains_key(*name) {
            Ok(format!("ERROR: Entity with name {} already exists; we can't handle duplicates yet!", *name))
        } else { 
            // Create a new entity with this name and register it locally 
            let entity = state.engine.new_entity(*name);
            let entity_str = entity.to_string();
            
            state.entities.insert(entity.name.clone(), entity);
            Ok(format!("Added entity: {}", entity_str))  
        }
    } else {
        Err(ParseError::new(tokens.last().unwrap_or(&""), tokens))
    }
} 

/// Parameters: <entity_name> (!TODO eventually EntityView?)
pub(super) fn command_delete_entity(tokens: &Vec<&str>, state: &mut PanicState) -> Result<String, ParseError> {
    // Validate format 
    if tokens.len() != 2 {
        return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    }

    return if let Some(name) = tokens.get(1) {
        // Try to remove an entity with this name 
        match state.entities.remove(&String::from(*name)) {
            Some(entity) => {
                // Remove it from the engine
                match state.engine.delete_entity(&entity) {
                    Ok(e) => Ok(format!("Removed entity: {}", e.get_name())), 
                    Err(()) => Ok(format!("ERROR: entity with name {} exists locally but not within the engine!", entity.get_name()))
                }
            }, 
            None => {
                // No such entity exists! 
                println!("Entities: {}", state.entities.keys().map(|k| k.as_str()).collect::<Vec<&str>>().join(", "));
                Ok(format!("ERROR: no entity named {} exists!", *name))
            }
        }
    } else {
        Err(ParseError::new(tokens.last().unwrap_or(&""), tokens))
    }
}

pub(super) fn command_list_entities(_tokens: &Vec<&str>, state: &mut PanicState) -> Result<String, ParseError> {
    // Ignore any trailing tokens - this can't fail at the parser level 
    Ok(state.engine.list_entities().iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
}