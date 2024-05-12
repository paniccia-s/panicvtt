use crate::{panic_state::PanicState, parse_error::ParseError};

/// Parameters: <entity_name>
pub(super) fn command_new_entity(tokens: &Vec<&str>, state: &mut PanicState) -> Result<String, ParseError> {
    // Validate parameter count
    return if let Some(name) = tokens.get(1) {
        // !TODO do not allow name duplicates until we can resolve them through the webpage
        if state.entities.contains_key(*name) {
            Ok(format!("ERROR: Entity with name {} already exists; we can't handle duplicates yet!", *name))
        } else { 
            // Create a new entity with this name and register it locally 
            let entity = state.engine.new_entity(*name);
            let entity_str = entity.to_string();
            
            state.entities.insert(String::from(entity.get_name()), entity.get_uuid());
            Ok(format!("Added entity: {}", entity_str))  
        }
    } else { 
        return Err(ParseError::from_wrong_num_args(     // !TODO idk about this unwrap_or() behavior here. 
            tokens, 2, tokens.len().try_into().unwrap_or(u8::MAX)));
    }  
} 

/// Parameters: <entity_name> (!TODO eventually EntityView?)
pub(super) fn command_delete_entity(tokens: &Vec<&str>, state: &mut PanicState) -> Result<String, ParseError> {
    // Validate parameter count
    return if let Some(name) = tokens.get(1) {
        // Try to remove an entity with this name 
        match state.entities.remove(&String::from(*name)) {
            Some(entity) => {
                // Remove it from the engine
                match state.engine.delete_entity(entity) {
                    Ok(e) => Ok(format!("Removed entity: {}", e.get_name())), 
                    Err(()) => Ok(format!("ERROR: entity with name {} exists locally but not within the engine!", *name))
                }
            }, 
            None => {
                // No such entity exists! 
                println!("Entities: {}", state.entities.keys().map(|k| k.as_str()).collect::<Vec<&str>>().join(", "));
                Ok(format!("ERROR: no entity named {} exists!", *name))
            }
        }
    } else {
        return Err(ParseError::from_wrong_num_args(     // !TODO idk about this unwrap_or() behavior here. 
            tokens, 2, tokens.len().try_into().unwrap_or(u8::MAX)));
    }
}

pub(super) fn command_list_entities(_tokens: &Vec<&str>, state: &mut PanicState) -> Result<String, ParseError> {
    // Ignore any trailing tokens - this can't fail at the parser level 
    Ok(state.engine.list_entities().iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", "))
}

pub(super) fn command_get_entity_ability(_tokens: &Vec<&str>, _state: &mut PanicState) -> Result<String, ParseError> {
    // // Validate format 
    // if tokens.len() != 3 {
    //     return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    // }

    // let name = tokens.get(1).unwrap();
    // let stat = tokens.get(2).unwrap();

    todo!("Refactoring ParseError so this isn't done yet");

    //Ok(String::new())
}