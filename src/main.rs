mod api;
mod config;
mod utils;
mod args;

use std::env;

use crate::args::*;
use crate::config::build_cfg;
use crate::api::{time_get, task_get, time_track};

// wrap main logic inside of run so we can print ArgErrors to stdout in readable format
// if we return Result<(), ArgError> from main, the error is printed in Debug format
fn run() -> Result<(), ArgError> {
    let cfg = build_cfg();
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        return Err(ArgError::ArgCount("Expects at least two arguments".to_string()))
    }
    let res = match args[0].as_str() {
        "timeget" => {
            let arg: TimeGet = match args[1].as_str() {
                "today" => TimeGet::Today,
                "week" => TimeGet::Week,
                "yesterday" => TimeGet::Yesterday,
                _ => return Err(ArgError::ArgValue("Invalid second argument for first argument 'timeget'. Only 'today', 'week' and 'yesterday' are valid!".to_string())) 
            };
            time_get(arg, &cfg)
        }
        "taskget" => {
            let arg: TaskGet = match args[1].as_str() {
                "last" => TaskGet::Last,
                "sprint" => {
                    TaskGet::Sprint
                },
                _ => return Err(ArgError::ArgValue("Invalid second argument for first argument 'taskget'. Only 'last' and 'sprint' are valid!".to_string())) 
            };
            task_get(arg, &cfg) 
        }
        "timetrack" => {
            if args.len() != 3 {
                return Err(ArgError::ArgCount("Invalid number of arguments for 'timetrack'!".to_string())) 
            }
            let duration = args[2].parse::<u32>()?;
            let arg: TimeTrack = match args[1].as_str() {
                "last" => {
                    TimeTrack {
                        mode: TimeTrackFirstArg::Last,
                        duration
                    }
                },
                _ => TimeTrack {
                    mode: TimeTrackFirstArg::TaskId(&args[1]),
                    duration
                }
            };
            time_track(arg, &cfg)
        }
        _ => return Err(ArgError::ArgValue("Invalid first argument! Only 'timeget', 'taskget' and 'timetrack' are valid!".to_string())) 
    };
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("[REQUEST ERROR] {}", e)
    };
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e)
    }
}
