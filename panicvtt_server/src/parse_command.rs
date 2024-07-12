// use panicvtt_engine::entities::abilities::{Ability, AbilityScoreIntType, AbilityScores};

// use crate::{panic_state::PanicState, parse_error::ParseError};

// /// Parameters: <entity_name>
// pub(super) fn command_new_entity(tokens: &[&str], state: &mut PanicState) -> Result<String, ParseError> {
//     // Validate parameter count
//     return if let Some(name) = tokens.get(1) {
//         // !TODO do not allow name duplicates until we can resolve them through the webpage
//         if state.entities.contains_key(*name) {
//             Ok(format!(
//                 "ERROR: Entity with name {} already exists; we can't handle duplicates yet!",
//                 *name
//             ))
//         } else {
//             // If abilities are provided, parse and deliver them
//             if tokens.len() > 2 {
//                 if let (
//                     Some(str_str), Some(dex_str), Some(con_str),
//                     Some(int_str), Some(wis_str), Some(cha_str),
//                 ) = (
//                     tokens.get(2), tokens.get(3), tokens.get(4),
//                     tokens.get(5), tokens.get(6), tokens.get(7),
//                 ) {
//                     // Now attempt to parse them into ints
//                     if let (Ok(str), Ok(dex), Ok(con), Ok(int), Ok(wis), Ok(cha)) = (
//                         str_str.parse::<AbilityScoreIntType>(), dex_str.parse::<AbilityScoreIntType>(),
//                         con_str.parse::<AbilityScoreIntType>(), int_str.parse::<AbilityScoreIntType>(),
//                         wis_str.parse::<AbilityScoreIntType>(), cha_str.parse::<AbilityScoreIntType>(),
//                     ) {
//                         // Create a new entity with this name and ability set and register it locally
//                         let abilities = AbilityScores::new(str, dex, con, int, wis, cha);
//                         let entity = state.engine.new_entity_with_abilities(name, abilities);
//                         let entity_str = entity.to_string();
    
//                         state.entities.insert(String::from(entity.get_name()), entity.get_uuid());
//                         Ok(format!("Added entity: {}", entity_str))
//                     } else {
//                         let s = format!("{} {} {} {} {} {}", str_str, dex_str, con_str, int_str, wis_str, cha_str);
//                         Err(ParseError::from_syntax_error(tokens, &s))
//                     }
//                 } else { 
//                     Err(ParseError::from_wrong_num_args(tokens, 7, tokens.len().try_into().unwrap_or(u8::MAX)))
//                 }
//             }
//             else {
//                 // Create a new entity with this name and default abilities and register it locally
//                 let entity = state.engine.new_entity(name);
//                 let entity_str = entity.to_string();

//                 state.entities.insert(String::from(entity.get_name()), entity.get_uuid());
//                 Ok(format!("Added entity: {}", entity_str))
//             }
//         }
//     } else {// !TODO idk about this unwrap_or() behavior here.
//         return Err(ParseError::from_wrong_num_args(tokens, 2, tokens.len().try_into().unwrap_or(u8::MAX),
//         ));
//     };
// }

// /// Parameters: <entity_name> (!TODO eventually EntityView?)
// pub(super) fn command_delete_entity(tokens: &[&str], state: &mut PanicState) -> Result<String, ParseError> {
//     // Validate parameter count
//     return if let Some(name) = tokens.get(1) {
//         // Try to remove an entity with this name
//         match state.entities.remove(&String::from(*name)) {
//             Some(entity) => {
//                 // Remove it from the engine
//                 match state.engine.delete_entity(entity) {
//                     Some(e) => Ok(format!("Removed entity: {}", e.get_name())),
//                     None => Ok(format!(
//                         "ERROR: entity with name {} exists locally but not within the engine!",
//                         *name
//                     )),
//                 }
//             }
//             None => {
//                 // No such entity exists!
//                 println!("Entities: {}",
//                     state.entities.keys()
//                         .map(|k| k.as_str())
//                         .collect::<Vec<&str>>()
//                         .join(", ")
//                 );
//                 Ok(format!("ERROR: no entity named {} exists!", *name))
//             }
//         }
//     } else {
//         return Err(ParseError::from_wrong_num_args(
//             // !TODO idk about this unwrap_or() behavior here.
//             tokens,
//             2,
//             tokens.len().try_into().unwrap_or(u8::MAX),
//         ));
//     };
// }

// pub(super) fn command_list_entities(_tokens: &[&str], state: &mut PanicState,
// ) -> Result<String, ParseError> {
//     // Ignore any trailing tokens - this can't fail at the parser level
//     Ok(state.engine.list_entities().iter()
//         .map(|e| e.to_string())
//         .collect::<Vec<String>>()
//         .join(", "))
// }

// pub(super) fn command_get_entity_ability(tokens: &[&str], state: &mut PanicState) -> Result<String, ParseError> {
//     return if let (Some(name), Some(ability_str)) = (tokens.get(1), tokens.get(2)) {
//         // Try to match an Entity with this name
//         if let Some(uuid) = state.entities.get(*name) {
//             // See if the Ability is valid
//             if let Ok(ability) = Ability::from_str(ability_str) {
//                 // Go for it
//                 let d = state.engine.get_ability_score(*uuid, ability).unwrap();
//                 Ok(format!("{} {} = {}", *name, ability_str, d))
//             } else {
//                 Ok(format!("ERROR: no ability named {}!", *ability_str))
//             }
//         } else {
//             Ok(format!("ERROR: no entity named {} exists!", *name))
//         }
//     } else {
//         // !TODO idk about this unwrap_or() behavior here.
//         return Err(ParseError::from_wrong_num_args(tokens, 3, tokens.len().try_into().unwrap_or(u8::MAX),
//         ));
//     };
// }

// pub(super) fn command_get_entity_abilities(tokens: &[&str], state: &mut PanicState) -> Result<String, ParseError> {
//     return if let Some(name) = tokens.get(1) {
//         // Try to match an Entity with this name
//         if let Some(uuid) = state.entities.get(*name) {
//             // Get the abilities
//             let abilities = state.engine.get_ability_scores(*uuid).unwrap();
//             Ok(format!("{}: [{}]", *name, abilities))
//         } else {
//             Ok(format!("ERROR: no entity named {} exists!", *name))
//         }
//     } else {
//         // !TODO idk about this unwrap_or() behavior here.
//         Err(ParseError::from_wrong_num_args(tokens, 2, tokens.len().try_into().unwrap_or(u8::MAX)))
//     };
// }


// #[cfg(test)]
// mod tests {
//     use panicvtt_engine::engine::Engine;

//     use crate::parse_error::ParseErrorKind;

//     use super::*;

//     #[test]
//     fn new_entity_happy_path() {
//         let mut state = PanicState::new(Engine::new());

//         // Happy-path default instantiation 
//         let tokens = vec!["new_entity", "David"];
//         let insertion = command_new_entity(&tokens, &mut state);
        
//         assert!(insertion.is_ok());
//         let res_str = insertion.unwrap(); 
//         assert!(res_str.starts_with("Added entity: Entity David (uuid ..."));
//         assert!(res_str.ends_with(')')); 

//         // Happy-path custom-attribute instantiation 
//         let tokens = vec!["new_entity", "Rick", "1", "2", "3", "4", "5", "6"];
//         let insertion = command_new_entity(&tokens, &mut state);

//         assert!(insertion.is_ok());
//         let res_str = insertion.unwrap();
//         assert!(res_str.starts_with("Added entity: Entity Rick (uuid ..."));
//         assert!(res_str.ends_with(')'));
//     }

//     #[test]
//     fn new_entity_wrong_num_args() {
//         let mut state = PanicState::new(Engine::new());
 
//         // Wrong number of arguments 
//         let mut tokens = vec!["new_entity" ];
//         let insertion = command_new_entity(&tokens, &mut state);
        
//         assert!(insertion.is_err());
//         let res_err = insertion.unwrap_err();

//         match res_err.error_kind {
//             ParseErrorKind::WrongNumArgs { expected_num, actual_num } => {
//                 assert!(expected_num == 2 && actual_num == 1);
//             }, 
//             ParseErrorKind::SyntaxError { bad_token: _ } => panic!()
//         };

//         let new_tokens = ["1", "2", "3", "4", "5"];
//         tokens.push("Syd");

//         for i in 0..5 {
//             // Add some custom ability scores, but not enough 
//             tokens.push(*new_tokens.get(i).unwrap());

//             // Reset the state so we don't repeat actors
//             let mut state = PanicState::new(Engine::new());
            
//             let insertion = command_new_entity(&tokens, &mut state);
//             assert!(insertion.is_err()); 

//             match insertion.unwrap_err().error_kind {
//                 ParseErrorKind::WrongNumArgs { expected_num, actual_num } => {
//                     assert!(expected_num == 7 && actual_num == (2 + i + 1) as u8);
//                 }, 
//                 ParseErrorKind::SyntaxError { bad_token: _ } => panic!()
//             };
//         }
//     }

//     #[test]
//     fn new_entity_duplicate_name() {
//         let mut state = PanicState::new(Engine::new());

//         // Insert one entity, then do it again 
//         let tokens = vec!["new_entity", "Nick"];
//         let _ok = command_new_entity(&tokens, &mut state); 
//         assert!(_ok.is_ok());

//         let insertion = command_new_entity(&tokens, &mut state);
//         assert!(insertion.is_ok()); 

//         assert_eq!(insertion.unwrap(), "ERROR: Entity with name Nick already exists; we can't handle duplicates yet!")
//     }

//     #[test]
//     fn new_entity_non_u8_vals() {
//         let mut state = PanicState::new(Engine::new());

//         let tokens_lists = [
//             vec!["new_entity", "1", "STR", "10", "10", "10", "10", "10"],
//             vec!["new_entity", "2", "10", "DEX", "10", "10", "10", "10"],
//             vec!["new_entity", "3", "10", "10", "CON", "10", "10", "10"],
//             vec!["new_entity", "4", "10", "10", "10", "INT", "10", "10"],
//             vec!["new_entity", "5", "10", "10", "10", "10", "WIS", "10"],
//             vec!["new_entity", "6", "10", "10", "10", "10", "10", "CHA"],
//         ];
//         let err_list = vec![
//             "STR 10 10 10 10 10", "10 DEX 10 10 10 10", "10 10 CON 10 10 10", 
//             "10 10 10 INT 10 10", "10 10 10 10 WIS 10", "10 10 10 10 10 CHA"
//         ];

//         for iter in tokens_lists.iter().zip(err_list) {
//             let (tokens, err_token) = iter;
//             let insertion = command_new_entity(tokens, &mut state);
//             assert!(insertion.is_err()); 

//             match insertion.unwrap_err().error_kind {
//                 ParseErrorKind::WrongNumArgs { expected_num: _, actual_num: _ } => panic!(), 
//                 ParseErrorKind::SyntaxError { bad_token } => {
//                     assert_eq!(bad_token, String::from(err_token));
//                 }
//             };
//         }
//     }
// }
