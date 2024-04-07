use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimeEntries {
    pub data: Vec<TimeEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TimeEntry {
    id: String,
    pub task: Option<Task>,
    start: String,
    pub end: String,
    duration: String,
    task_url: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub id: String,
    pub name: String,
    status: Status
}

#[derive(Debug, Deserialize, Clone)]
struct Status {
    status: String
}


pub fn calculate_time(entries: TimeEntries) -> f32 {
    // calculate tracked time in hours
    entries.data.iter().map(|entry| {
        entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32 / 60f32
    }).sum()
}

pub mod request {
    use super::*;
    use crate::config::Cfg;
    use reqwest::blocking::Client;
    use reqwest::Method;
    use serde_json::{to_string, from_str};

    pub fn make_get_request(cfg: &Cfg, start: i64, end: i64, url: String) -> Result<TimeEntries, reqwest::Error> { // building request
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
        let time_entries: TimeEntries = from_str(&res).unwrap();
        Ok(time_entries)
    }

    pub fn make_post_request(cfg: &Cfg, url: String, body: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
        let client: Client = Client::new();
        let req_body = to_string(&body)?;
        let req = client.request(Method::POST , url).header("content-type", "application/json").header("Authorization", cfg.token.clone()).body(req_body.clone());

        let status = req.send()?.status();
        let success = status.is_success();

        if success {
            Ok(())
        } else {
            Err(format!("Request failed with status code: {}", status.as_u16()).into())
        }
    }
}

pub mod display {
    use chrono::DateTime;
    use chrono::Utc;

    use super::TimeEntry;

    pub const HOURGLASS: char = '\u{231B}';
    pub const ERROR: char = '\u{1F6AB}';
    const ALARM_CLOCK: char = '\u{23F0}';
    const CHECKMARK: char = '\u{2705}';
    const LABEL: char = '\u{1F4CA}';


    pub fn fmt_time(hours: f32) -> String {
        if hours.fract() == 0.0 {
            format!("{:.0}h", hours)
        } else {
            format!("{:.2}h", hours)
        }
    }
    pub fn fmt_task(entry: &TimeEntry) -> String {
        let mut out = String::with_capacity(64);
        let last_entry_ts_ms = entry.end.parse::<i64>().unwrap();
        let last_entry_ts_s = last_entry_ts_ms / 1000;
        let last_entry_ts_ns = (last_entry_ts_ms % 1000) * 1_000_000;
        let last_entry_dt = DateTime::from_timestamp(last_entry_ts_s, last_entry_ts_ns as u32).unwrap();

        let last_tracked_in_mins = Utc::now().signed_duration_since(last_entry_dt).num_minutes();

        if let Some(task) = &entry.task {
            out.push_str(&format!("{: <14}", &format!("{} [TASK]", CHECKMARK)));
            out.push_str(&format!(" {} ({})\n", task.name, entry.task_url.as_ref().unwrap()));
            out.push_str(&format!("{: <14}", &format!("{} [STATUS]", LABEL)));
            out.push_str(&format!(" {}\n", task.status.status));
        } else {
            out.push_str("No task associated with this entry\n");
        }

        out.push_str(&format!("{: <14}", &format!("{} [LAST ENTRY]", ALARM_CLOCK)));
        out.push_str(&format!(" {} minutes ({}) ago\n", last_tracked_in_mins, fmt_time(last_tracked_in_mins as f32 / 60f32)));
        out.push_str(&format!("{: <14}", &format!("{} [DURATION]", HOURGLASS)));
        out.push_str(&format!(" {} minutes ({})\n", entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32, fmt_time(entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32 / 60f32)));
        out
    }
}
