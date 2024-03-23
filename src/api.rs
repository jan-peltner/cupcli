use std::collections::HashMap;

use crate::args::*;
use crate::config::Cfg;
use crate::utils::display::{fmt_task, fmt_time, HOURGLASS};
use crate::utils::request::{make_get_request, make_post_request};
use crate::utils::{calculate_time, TimeEntry};
use chrono::{Datelike, Days, Local, Timelike};
pub fn time_get(arg: TimeGet, cfg: &Cfg) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/time_entries",
        cfg.team_id
    );
    match arg {
        TimeGet::Today => {
            let local = Local::now().date_naive();
            let start = local.and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
            let end: i64 = Local::now().timestamp_millis();
            let res = make_get_request(&cfg, start, end, url);
            match res {
                Ok(res) => {
                    let res = calculate_time(res);
                    Ok(format!(
                        "{} Tracked time today: {} out of {}",
                        HOURGLASS,
                        fmt_time(res),
                        fmt_time(cfg.daily_quota)
                    ))
                }
                Err(e) => Err(e),
            }
        }
        TimeGet::Week => {
            let now = Local::now();
            let closest_past_monday = now
                .checked_sub_days(Days::new(now.weekday().num_days_from_monday().into()))
                .unwrap();
            let start = closest_past_monday
                .date_naive()
                .and_hms_opt(0, 0, 1)
                .unwrap()
                .timestamp_millis();
            let end = now.timestamp_millis();
            let res = make_get_request(&cfg, start, end, url);
            match res {
                Ok(res) => {
                    let res = calculate_time(res);
                    Ok(format!(
                        "{} Tracked time this week: {} out of {}",
                        HOURGLASS,
                        fmt_time(res),
                        fmt_time(cfg.daily_quota * 5f32)
                    ))
                }
                Err(e) => Err(e),
            }
        }
        TimeGet::Yesterday => {
            let now = Local::now();
            let yesterday = now.checked_sub_days(Days::new(1)).unwrap();
            let start = yesterday
                .date_naive()
                .and_hms_opt(0, 0, 1)
                .unwrap()
                .timestamp_millis();
            let end = yesterday
                .date_naive()
                .and_hms_opt(23, 59, 59)
                .unwrap()
                .timestamp_millis();
            let res = make_get_request(&cfg, start, end, url);
            match res {
                Ok(res) => {
                    let res = calculate_time(res);
                    Ok(format!(
                        "{} Tracked time yesterday: {} out of {}",
                        HOURGLASS,
                        fmt_time(res),
                        fmt_time(cfg.daily_quota)
                    ))
                }
                Err(e) => Err(e),
            }
        }
    }
}

pub fn task_get(arg: TaskGet, cfg: &Cfg) -> Result<String, reqwest::Error> {
    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/time_entries",
        cfg.team_id
    );
    let now = Local::now();
    match arg {
        TaskGet::Last => {
            let start_ndt = now
                .checked_sub_days(Days::new(cfg.look_behind))
                .unwrap()
                .date_naive()
                .and_hms_opt(0, 0, 1)
                .unwrap();
            let start_ts = start_ndt.timestamp_millis();
            let end = now.timestamp_millis();
            let res = make_get_request(&cfg, start_ts, end, url);
            match res {
                Ok(res) => {
                    if let Some(last_entry) = res.data.last() {
                        Ok(fmt_task(&last_entry))
                    } else {
                        Ok(format!(
                            "No tasks tracked since {}",
                            start_ndt.format("%d/%m/%Y %H:%M:%S")
                        ))
                    }
                }
                Err(e) => Err(e),
            }
        }
        TaskGet::Sprint => todo!(),
    }
}

// Gets the last time entry without handling the response
fn task_get_last_internal(cfg: &Cfg) -> Result<TimeEntry, reqwest::Error> {
    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/time_entries",
        cfg.team_id
    );
    let now = Local::now();
    let start_ndt = now
        .checked_sub_days(Days::new(cfg.look_behind))
        .unwrap()
        .date_naive()
        .and_hms_opt(0, 0, 1)
        .unwrap();
    let start_ts = start_ndt.timestamp_millis();
    let end = now.timestamp_millis();
    let res = make_get_request(&cfg, start_ts, end, url)?
        .data
        .into_iter()
        .last()
        .unwrap(); // unsafe unwrap
    Ok(res)
}

#[allow(dead_code)]
pub fn time_track(arg: TimeTrack, cfg: &Cfg) -> Result<String, reqwest::Error> {
    match arg.mode {
       TimeTrackFirstArg::Last => {
        let time_entry = task_get_last_internal(cfg)?;
        if let Some(task) = time_entry.task {
            let end = Local::now().with_second(0).unwrap().timestamp_millis();
            let duration = match arg.duration {
                0 => end - time_entry.end.parse::<i64>().unwrap(),
                _ => arg.duration as i64 * 60 * 1000 // convert minutes to milliseconds
            };
            let start = end - duration;
            let mut body = HashMap::with_capacity(4);
            body.insert("start".to_string(), start.to_string());
            body.insert("end".to_string(), end.to_string());
            body.insert("duration".to_string(), duration.to_string());
            body.insert("tid".to_string(), task.id);
            let url = format!("https://api.clickup.com/api/v2/team/{}/time_entries", cfg.team_id);
            if make_post_request(cfg, url, body).is_ok() {
               Ok(format!("{} Tracked {} for task {}",HOURGLASS , fmt_time(duration as f32 / 1000f32 / 60f32 / 60f32), task.name))
            } else {
                panic!("Failed to track time for task {}", task.name)
            }
        } else {
            panic!("No task id found for last time entry")
        }
       }
       TimeTrackFirstArg::TaskId(_id) => {
           todo!()
       }
    }
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
