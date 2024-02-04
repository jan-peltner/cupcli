use crate::config::Cfg;
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

pub fn make_request(cfg: &Cfg, start: i64, end: i64) -> Result<f32, reqwest::Error> {
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

pub fn fmt_time(time: f32) -> String {
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