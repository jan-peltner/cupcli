mod request_utils;

use crate::config::build_cfg;
use request_utils::{make_request, fmt_time};
use chrono::{Local, Datelike, Days};


pub fn time_get() -> Result<String, reqwest::Error> {
    let cfg = build_cfg();
    match cfg.arg.1.as_str() {
        "today" => {
            let local = Local::now().date_naive();
            let start = local.and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
            let curr: i64 = Local::now().timestamp_millis(); 
            let res = make_request(&cfg, start, curr);
            match res {
                Ok(res) => Ok(format!("Tracked time today: {} out of {}", fmt_time(res), fmt_time(cfg.daily_quota))),
                Err(e) => Err(e),
            }
        }
        "week" => {
            let now = Local::now();
            let closest_past_monday = now.checked_sub_days(Days::new(now.weekday().num_days_from_monday().into())).unwrap();
            let start = closest_past_monday.date_naive().and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
            let curr = now.timestamp_millis();
            let res = make_request(&cfg, start, curr);
            match res {
                Ok(res) => Ok(format!("Tracked time this week: {} out of {}", fmt_time(res), fmt_time(cfg.daily_quota * 5f32))),
                Err(e) => Err(e),
            }
        }
        _ => todo!(),
    }
}


#[allow(dead_code)]
pub fn time_track() -> Result<(), reqwest::Error> {
    todo!()
}

#[allow(dead_code)]
pub fn tasks_list() -> Result<(), reqwest::Error> {
    todo!()
}

#[allow(dead_code)]
pub fn task_set_status() -> Result<(), reqwest::Error> {
    todo!()
}

#[allow(dead_code)]
pub fn task_create_comment() -> Result<(), reqwest::Error> {
    todo!()
}





