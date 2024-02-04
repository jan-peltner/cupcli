use crate::config::{build_cfg, Cfg};
use chrono::{Local, Datelike, Days};
use reqwest::blocking::Client;
use reqwest::Method;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct TimeEntries {
    data: Vec<TimeEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct TimeEntry {
    id: String,
    start: String,
    end: String,
    duration: String,
}

pub fn get_time() -> Result<String, reqwest::Error> {
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

fn make_request(cfg: &Cfg, start: i64, end: i64) -> Result<f32, reqwest::Error> {
    // building request 
    let url = format!(
        "https://api.clickup.com/api/v2/team/{}/time_entries",
        cfg.team_id
    );
    let client = Client::new();
    let req = client
        .request(Method::GET, url)
        .header("content-type", "appication/json")
        .header("Authorization", cfg.token.clone());

    // adding query params to request
    let mut query_params: Vec<(String, String)> = Vec::new();
    query_params.push(("start_date".to_string(), format!("{}", start)));
    query_params.push(("end_date".to_string(), format!("{}", end)));
    let res = req.query(&query_params).send()?.text()?;
    let time_entries: TimeEntries = serde_json::from_str(&res).unwrap(); 
    Ok(calculate_time(time_entries))
}

fn fmt_time(time: f32) -> String {
    if time.fract() == 0.0 {
        format!("{:.0}h", time)
    } else {
        format!("{:.2}h", time)
    }
}

fn calculate_time(mut entries: TimeEntries) -> f32 {
    // calculate tracked time in hours
    entries.data.iter_mut().map(|entry| {
        entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32 / 60f32
    }).sum()
}