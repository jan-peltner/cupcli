use std::fmt;
use std::num::ParseIntError;

use crate::utils::display::ERROR;

pub enum TimeGet {
    Today,
    Week,
    Yesterday,
}

pub enum TaskGet {
    Last,
    Sprint,
}

#[derive(Debug)]
pub struct TimeTrack<'a> {
    pub mode: TimeTrackMode<'a>,
    pub flags: Vec<TimeTrackFlag<'a>>,
}

#[derive(Debug)]
pub enum TimeTrackFlag<'a> {
    Description(&'a str),
    Duration(u32)
}

#[derive(Debug, PartialEq)]
pub enum TimeTrackMode<'a> {
    Last,
    Free,
    TaskId(&'a str),
}

#[derive(Debug, Clone)]
pub enum ArgError {
   ArgCount(String),
   ArgValue(String)
}

impl std::error::Error for ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut prefix = String::from(format!("{} [ARGUMENT ERROR] ", ERROR));
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