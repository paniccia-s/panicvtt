use std::fmt::Display;

use panicvtt_engine::engine::Engine;

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
pub(super) fn command_new_entity(tokens: &Vec<&str>, engine: &mut Engine) -> Result<String, ParseError> {
    // Make sure everything is formatted correctly 
    if tokens.len() != 2 {
        return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    }

    return if let Some(name) = tokens.get(1) {
        let entity = engine.create_entity(*name);
        Ok(format!("Added entity: {}", entity))
    } else {
        Err(ParseError::new(tokens.last().unwrap_or(&""), tokens))
    }
} 

/// Parameters: <entity_name> (!TODO eventually EntityView?)
pub(super) fn command_delete_entity(tokens: &Vec<&str>, engine: &mut Engine) -> Result<String, ParseError> {
    // Validate format 
    if tokens.len() != 2 {
        return Err(ParseError::new(tokens.last().unwrap_or(&""), tokens));
    }

    return if let Some(name) = tokens.get(1) {
        Ok(format!("Removed entity: {}", name))
    } else {
        Err(ParseError::new(tokens.last().unwrap_or(&""), tokens))
    }
}