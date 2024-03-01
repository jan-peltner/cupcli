use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TimeEntries {
    pub data: Vec<TimeEntry>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TimeEntry {
    id: String,
    task: Option<Task>,
    start: String,
    end: String,
    duration: String,
    task_url: Option<String>
}

#[derive(Debug, Deserialize)]
struct Task {
    name: String,
    status: Status 
}

#[derive(Debug, Deserialize)]
struct Status {
    status: String
}


pub fn calculate_time(mut entries: TimeEntries) -> f32 {
    // calculate tracked time in hours
    entries.data.iter_mut().map(|entry| {
        entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32 / 60f32
    }).sum()
}

pub mod req {
    use super::*;
    use crate::config::Cfg;
    use reqwest::blocking::Client;
    use reqwest::Method;

    pub fn make_request(cfg: &Cfg, start: i64, end: i64, url: String) -> Result<TimeEntries, reqwest::Error> { // building request 
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
        Ok(time_entries)
    }
}

pub mod display {
    use chrono::DateTime;
    use chrono::Utc;
    
    use super::TimeEntry;
    pub fn fmt_time(hours: f32) -> String {
        if hours.fract() == 0.0 {
            format!("{:.0}h", hours)
        } else {
            format!("{:.2}h", hours)
        }
    }
    pub fn fmt_task(entry: &TimeEntry) -> String {
        let mut out = String::new(); 

        let last_entry_ts_ms = entry.end.parse::<i64>().unwrap();
        let last_entry_ts_s = last_entry_ts_ms / 1000;
        let last_entry_ts_ns = (last_entry_ts_ms % 1000) * 1_000_000;
        let last_entry_dt = DateTime::from_timestamp(last_entry_ts_s, last_entry_ts_ns as u32).unwrap();

        let last_tracked_in_mins = Utc::now().signed_duration_since(last_entry_dt).num_minutes();

        if let Some(task) = &entry.task {
            out.push_str(&format!("{: <12}", "[TASK]"));
            out.push_str(&format!(" {} ({})\n", task.name, entry.task_url.as_ref().unwrap()));
            out.push_str(&format!("{: <12}", "[STATUS]"));
            out.push_str(&format!(" {}\n", task.status.status));
        } else {
            out.push_str("No task associated with this entry\n");
        }

        out.push_str(&format!("{: <12}", "[LAST ENTRY]"));
        out.push_str(&format!(" {} minutes ({}) ago\n", last_tracked_in_mins, fmt_time(last_tracked_in_mins as f32 / 60f32))); 
        out.push_str(&format!("{: <12}", "[DURATION]"));
        out.push_str(&format!(" {} minutes ({})\n", entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32, fmt_time(entry.duration.parse::<f32>().unwrap() / 1000f32 / 60f32 / 60f32)));
        out
    }
}
