mod api;
mod config;
mod utils;
mod args;

use std::env;
use std::fmt;
use std::num;

use crate::api::{time_get, task_get, time_track};
use crate::args::*;

#[derive(Debug)]
struct ArgError {
    msg: String
}

impl std::error::Error for ArgError {}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument error: {}", self.msg)
    }
}

impl From<num::ParseIntError> for ArgError {
    fn from(e: num::ParseIntError) -> Self {
        ArgError { msg: e.to_string() }
    }
}

fn main() -> Result<(), ArgError> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        return Err(ArgError { msg: "Expects at least two arguments".to_string() })
    }
    let res = match args[0].as_str() {
        "timeget" => {
            let arg: TimeGet = match args[1].as_str() {
                "today" => TimeGet::Today,
                "week" => TimeGet::Week,
                "yesterday" => TimeGet::Yesterday,
                _ => return Err(ArgError { msg: "Invalid second argument for first argument 'timeget'. Only 'today', 'week' and 'yesterday' are valid!".to_string() })
            };
            time_get(arg)
        }
        "taskget" => {
            let arg: TaskGet = match args[1].as_str() {
                "last" => TaskGet::Last,
                "sprint" => {
                    TaskGet::Sprint
                },
                _ => return Err(ArgError { msg: "Invalid second argument for first argument 'taskget'. Only 'last' and 'sprint' are valid!".to_string() }) 
            };
            task_get(arg) 
        }
        "timetrack" => {
            if args.len() != 3 {
                return Err(ArgError { msg: "Invalid number of arguments for 'timetrack'!".to_string() }) 
            }
            let duration = args[2].parse::<u32>()?;
            let arg: TimeTrack = match args[1].as_str() {
                "last" => {
                    TimeTrack {
                        mode: TimeTrackFirstArg::Last,
                        duration: duration,
                    }
                },
                _ => TimeTrack {
                    mode: TimeTrackFirstArg::TaskId(&args[1]),
                    duration: duration,
                }
            };
            time_track(arg)
        }
        _ => return Err(ArgError { msg: "Invalid first argument! Only 'timeget', 'taskget' and 'timetrack' are valid!".to_string() }) 
    };
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => println!("An error occurred: {}", e)
    };
    Ok(())
}
