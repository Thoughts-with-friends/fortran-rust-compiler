use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ErrorKind {
    UnexpectedEOF,
    UnknownCharacter(char),
    MessageWithLocation(usize, &'static str),
}

#[macro_export]
macro_rules! error_at {
    ($data:expr, $message:tt) => {
      eprintln!("{}")
      eprintln!($message);
    };
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedEOF => write!(f, "msg: {{ \"Unexpected EOF\" }}"),
            ErrorKind::UnknownCharacter(ch) => {
                write!(f, "{{ \"msg\": \"Unknown character {}.\" }}", ch)
            }
            ErrorKind::MessageWithLocation(line, message) => {
                write!(f, "{{ \"location\": {}, \"msg\": \"{}\" }}", line + 1, message)
            }
        }
    }
}
