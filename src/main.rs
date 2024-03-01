mod api;
mod config;
mod utils;
mod args;

use std::env;

use crate::api::{time_get, task_get, time_track};
use crate::args::*;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        panic!("Cupcli expects exactly two arguments!")
    }
    let res = match args[0].as_str() {
        "timeget" => {
            let arg: TimeGet = match args[1].as_str() {
                "today" => TimeGet::Today,
                "week" => TimeGet::Week,
                "yesterday" => TimeGet::Yesterday,
                _ => panic!("Invalid second argument for first argument 'timeget'. Only 'today', 'week', and 'yesterday' are valid!")
            };
            time_get(arg)
        }
        "taskget" => {
            let arg: TaskGet = match args[1].as_str() {
                "last" => TaskGet::Last,
                "sprint" => {
                    TaskGet::Sprint
                },
                _ => panic!("Invalid second argument for first argument 'taskget'. Only 'last' and 'sprint' are valid!")
            };
            task_get(arg) 
        }
        "timetrack" => {
            let arg: TimeTrack = match args[1].as_str() {
                "last" => TimeTrack::Last,
                _ => TimeTrack::TaskId(&args[1])
            };
            time_track(arg)
        }
        _ => panic!("Timetrack has not been implemented yet!")
    };
    match res {
        Ok(res) => println!("{}", res),
        Err(e) => println!("An error occurred: {}", e)
    }
}
