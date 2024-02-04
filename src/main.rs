mod request;
mod config;
use crate::request::time_get;

fn main() {
    match time_get() {
        Ok(time) => println!("{}", time),
        Err(e) => println!("{}", e),
    }
}
