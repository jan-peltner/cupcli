use crate::args::*;
use crate::config::build_cfg;
use crate::utils::calculate_time;
use crate::utils::network::make_request;
use crate::utils::display::{fmt_time, fmt_task};
use chrono::{Local, Datelike, Days};

pub fn time_get(arg: TimeGet) -> Result<String, reqwest::Error> {
    let cfg = build_cfg();
    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/time_entries",
        cfg.team_id
    );
    match arg {
        TimeGet::Today => {
            let local = Local::now().date_naive();
            let start = local.and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
            let end: i64 = Local::now().timestamp_millis(); 
            let res = make_request(&cfg, start, end, url);
            match res {
                Ok(res) => {
                    let res = calculate_time(res);
                    Ok(format!("Tracked time today: {} out of {}", fmt_time(res), fmt_time(cfg.daily_quota)))
                },
                Err(e) => Err(e),
            }
        }
        TimeGet::Week => {
            let now = Local::now();
            let closest_past_monday = now.checked_sub_days(Days::new(now.weekday().num_days_from_monday().into())).unwrap();
            let start = closest_past_monday.date_naive().and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
            let end = now.timestamp_millis();
            let res = make_request(&cfg, start, end, url);
            match res {
                Ok(res) => {
                    let res = calculate_time(res);
                    Ok(format!("Tracked time this week: {} out of {}", fmt_time(res), fmt_time(cfg.daily_quota * 5f32)))
                },
                Err(e) => Err(e),
            }
        }
        TimeGet::Yesterday => {
            let now = Local::now();
            let yesterday = now.checked_sub_days(Days::new(1)).unwrap();
            let start = yesterday.date_naive().and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
            let end = yesterday.date_naive().and_hms_opt(23, 59, 59).unwrap().timestamp_millis();
            let res = make_request(&cfg, start, end, url);
            match res {
                Ok(res) => {
                    let res = calculate_time(res);
                    Ok(format!("Tracked time yesterday: {} out of {}", fmt_time(res), fmt_time(cfg.daily_quota)))
                },
                Err(e) => Err(e),
            }
        }
    }
}

pub fn task_get(arg: TaskGet) -> Result<String, reqwest::Error> {
    let cfg = build_cfg();
    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/time_entries",
        cfg.team_id
    );
    let now = Local::now();
    match arg {
        TaskGet::Last => {
            let start_ndt = now.checked_sub_days(Days::new(cfg.look_behind)).unwrap().date_naive().and_hms_opt(0, 0, 1).unwrap();
            let start_ts = start_ndt.timestamp_millis();
            let end = now.timestamp_millis();
            let res = make_request(&cfg, start_ts, end, url);
            match res {
                Ok(res) => {
                    if let Some(last_entry) = res.data.last() {
                        Ok(fmt_task(&last_entry))
                    } else {
                        Ok(format!("No tasks tracked since {}", start_ndt.format("%d/%m/%Y %H:%M:%S")))
                    }
                },
                Err(e) => Err(e),
            }
        },
        TaskGet::Sprint => todo!()
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





