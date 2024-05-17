use std::fmt::Display;


#[derive(Debug)]
pub(super) enum ParseErrorKind {
    WrongNumArgs { expected_num: u8, actual_num: u8 },
    SyntaxError { bad_token: String }, 
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseErrorKind::WrongNumArgs { expected_num, actual_num } => {
                format!("Expected {} arguments, but received {}", expected_num, actual_num)
            }, 
            ParseErrorKind::SyntaxError { bad_token } => {
                format!("Syntax error on token {}", bad_token)
            }
        })
    }
}

#[derive(Debug)]
pub(super) struct ParseError { 
    pub(super) all_tokens: Vec<String>,
    pub(super) error_kind: ParseErrorKind
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {} (all tokens: [", self.error_kind)?;
        self.all_tokens.iter().try_for_each(|t| f.write_str(format!("{}, ", t).as_str()))?;
        write!(f, "]")
    }
}

impl std::error::Error for ParseError {}

impl ParseError {
    fn new(all_tokens: &[&str], error_kind: ParseErrorKind) -> Self {
        Self {
            all_tokens: all_tokens.iter().map(|s| { String::from(*s) }).collect(), 
            error_kind
        }
    }

    pub(super) fn from_syntax_error(all_tokens: &[&str], bad_token: &str) -> Self {
        Self::new(all_tokens, ParseErrorKind::SyntaxError{ bad_token: String::from(bad_token) })
    } 

    pub(super) fn from_wrong_num_args(all_tokens: &[&str], expected_num: u8, actual_num: u8) -> Self {
        Self::new(all_tokens, ParseErrorKind::WrongNumArgs { expected_num, actual_num })
    }
}
