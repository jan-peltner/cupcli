mod api;
mod args;
mod config;
mod utils;

use std::collections::HashSet;
use std::env;

use itertools::{Itertools, Either};

use crate::api::{task_get, time_get, time_track};
use crate::args::*;
use crate::config::build_cfg;

// wrap main logic inside of run so we can print ArgErrors to stdout in readable format
// if we return Result<(), ArgError> from main, the error is printed in Debug format
fn run() -> Result<(), ArgError> {
    let cfg = build_cfg();
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        return Err(ArgError::ArgCount(
            "Expects at least two arguments".to_string(),
        ));
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
        } "taskget" => {
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
            if args.len() < 2 {
                return Err(ArgError::ArgCount(
                    "Invalid number of arguments for 'timetrack'!".to_string(),
                ));
            }

            args.remove(0); // not optimal because we shift all elements by 1, but it's fine for now
            let valid_flags: HashSet<&str> = HashSet::from(["-d", "--description", "-D", "--duration"]);

            // separate the args vec into flags / options and the remaining args
            let (args, flags): (Vec<&str>, Vec<Result<TimeTrackFlag, ArgError>>) = args.chunks(2).partition_map(|chunk| {
                if let Some(v) = valid_flags.get(&chunk[0].as_str()) {
                    match *v {
                        "-D" | "--description" => Either::Right(Ok(TimeTrackFlag::Description(chunk[1].as_str()))),
                        "-d" | "--duration" => {
                            let parsed_v = chunk[1].parse::<u32>();
                            match parsed_v {
                                Ok(v) => Either::Right(Ok(TimeTrackFlag::Duration(v))),
                                Err(_) => Either::Right(Err(ArgError::ArgValue(format!("Invalid value for flag {}", chunk[0])))),
                            }
                        },
                        _ => unreachable!()

                    } // valid flag
                } else if chunk[0].starts_with("-") {
                    Either::Right(Err(ArgError::ArgValue(format!("Invalid flag: {}", chunk[0])))) // invalid flag
                }
                else {
                    Either::Left(chunk[0].as_str()) // positional arg
                }
            });
            let flag_err = flags.iter().find_map(|f| {
                match f {
                    Ok(_) => None,
                    Err(e) => Some(e)
                }
            });
            if flag_err.is_some() {
                return Err(flag_err.unwrap().clone())
            }
            let flags = flags.into_iter().map(|f| f.unwrap()).collect();

            let args = if args.is_empty() {
               TimeTrack {
                mode: TimeTrackMode::Free,
                flags
               }
            } else {
                match args[0] {
                    "last" => TimeTrack {
                        mode: TimeTrackMode::Last,
                        flags,
                    },
                    _ => TimeTrack {
                        mode: TimeTrackMode::TaskId(&args[0]),
                        flags,
                    },
                }
            };
            time_track(args, &cfg)
        }
        _ => {
            return Err(ArgError::ArgValue(
                "Invalid first argument! Only 'timeget', 'taskget' and 'timetrack' are valid!"
                    .to_string(),
            ))
        }
    };
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("[REQUEST ERROR] {}", e),
    };
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e)
    }
}
