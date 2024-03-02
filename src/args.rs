use std::fmt;
use std::num::ParseIntError;
pub enum TimeGet {
    Today,
    Week,
    Yesterday,
}

pub enum TaskGet {
    Last,
    Sprint,
}

pub struct TimeTrack<'a> {
    pub mode: TimeTrackFirstArg<'a>,
    pub duration: u32,
}

pub enum TimeTrackFirstArg<'a> {
    Last,
    TaskId(&'a str),
}

#[derive(Debug)]
pub enum ArgError {
   ArgCount(String),
   ArgValue(String) 
}

impl std::error::Error for ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut prefix = String::from("[ARGUMENT ERROR] ");
        match self {
            ArgError::ArgCount(msg) => {
                prefix.push_str("Invalid number of arguments: ");
                write!(f, "{}{}", prefix, msg)
            },
            ArgError::ArgValue(msg) => {
                prefix.push_str("Invalid argument value: ");
                write!(f, "{}{}", prefix, msg)
            }
        }
    }
}

impl From<ParseIntError> for ArgError {
    fn from(e: ParseIntError) -> Self {
        ArgError::ArgValue(e.to_string()) 
    }
}