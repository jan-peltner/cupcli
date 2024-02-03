use std::{env, fs};
pub mod config {
    use super::*;

    #[derive(Debug)]
    pub struct Cfg {
        pub teamid: String,
        pub token: String,
        pub arg: (Mode, String),
        pub daily_quota: f32,
    }

    #[derive(Debug)]
    pub enum Mode {
        TimeGet,
        TimeTrack,
    }

    impl FromIterator<(String, String)> for Cfg {
        fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
            /* needs fix - currently config gets parsed even if teamid and cu_auth are empty in cfg file */
            let mut cfg = Cfg {
                teamid: String::new(),
                token: String::new(),
                arg: (Mode::TimeGet, String::new()),
                daily_quota: 8.0,
            };
            for (key, value) in iter {
                match key.as_str() {
                    "teamid" => cfg.teamid = value,
                    "cu_auth" => cfg.token = value,
                    "dailyQuota" => cfg.daily_quota = value.parse::<f32>().unwrap_or(8.0),
                    "timeget" => cfg.arg = (Mode::TimeGet, value),
                    "timetrack" => cfg.arg = (Mode::TimeTrack, value),
                    _ => panic!("Could not parse config. Check config file and arguments!"),
                }
            }
            cfg
        }
    }

    fn parse_cfg() -> Vec<(String, String)> {
        let home = env::var("HOME").expect("Could not get $HOME env var; expose it first!");
        let cfg_str = fs::read_to_string(format!("{}/.config/cupcli/cfg", home)).expect(
            r#"
            ---------------------------------------------------------------------------
            Config file not found! 
            Please create ~/.config/cupcli/cfg and add the following:
            cu_auth={YOUR_CLICKUP_AUTH_TOKEN}
            teamid={YOUR_TEAM_ID} 
            ---------------------------------------------------------------------------
            "#,
        );
        let cfg: Vec<(String, String)> = cfg_str
            .lines()
            .map(|line| {
                let mut split = line.split("=");
                let key = split.next().unwrap().to_string();
                let value = split.next().unwrap().to_string();
                (key, value)
            })
            .collect();

        cfg
    }

    fn parse_args() -> (String, String) {
        let args: Vec<String> = env::args().skip(1).collect();
        if args.len() < 2 {
            panic!("Cupcli expects at least one argument one value!")
        }
        let mut args_out = (String::new(), String::new());
        for (idx, arg) in args.iter().enumerate() {
            if idx == 0 {
                match arg.as_str() {
                    "timeget" => args_out.0 = arg.to_string(),
                    "timetrack" => args_out.0 = arg.to_string(),
                    _ => panic!(
                        "Invalid argument! Only timeget <'today'|'week'> and timetrack <taskid> are valid arguments!"
                    ),
                }
            } else {
                match (args_out.0.as_str(), arg.as_str()) {
                    ("timeget", "today") => args_out.1 = arg.to_string(),
                    ("timeget", "week") => args_out.1 = arg.to_string(),
                    ("timeget", _) => panic!(
                        "Invalid value for argument 'timeget'. Only 'today' and 'week' are valid!"
                    ),
                    ("timetrack", _) => todo!(),
                    (_, _) => panic!("Invalid argument!"),
                }
            }
        }
        args_out
    }

    pub fn build_cfg() -> Cfg {
        let mut cfg = parse_cfg();
        cfg.push(parse_args());
        cfg.into_iter().collect()
    }
}

pub mod request {
    use crate::config::build_cfg;
    use chrono::{Local, Datelike, Days};
    use reqwest::blocking::{Client, RequestBuilder};
    use reqwest::Method;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TimeEntries {
        data: Vec<TimeEntry>,
    }
    #[derive(Debug, Deserialize)]
    struct TimeEntry {
        id: String,
        start: String,
        end: String,
        duration: String,
    }

    pub fn get_time() -> Result<String, reqwest::Error> {
        let cfg = build_cfg();
        let url = format!(
            "https://api.clickup.com/api/v2/team/{}/time_entries",
            cfg.teamid
        );
        let client = Client::new();
        let req = client
            .request(Method::GET, url)
            .header("content-type", "application/json")
            .header("Authorization", cfg.token);
        match cfg.arg.1.as_str() {
            "today" => {
                let local = Local::now().date_naive();
                let start = local.and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
                let curr: i64 = Local::now().timestamp_millis(); 
                let res = make_request(req, start, curr);
                match res {
                    Ok(res) => Ok(format!("Tracked time today: {} out of {}", format_time(res), format_time(cfg.daily_quota))),
                    Err(e) => Err(e),
                }
            }
            "week" => {
                let now = Local::now();
                let closest_past_monday = now.checked_sub_days(Days::new(now.weekday().num_days_from_monday().into())).unwrap();
                let start = closest_past_monday.date_naive().and_hms_opt(0, 0, 1).unwrap().timestamp_millis();
                let curr = now.timestamp_millis();
                let res = make_request(req, start, curr);
                match res {
                    Ok(res) => Ok(format!("Tracked time this week: {} out of {}", format_time(res), format_time(cfg.daily_quota * 5f32))),
                    Err(e) => Err(e),
                }
            }
            _ => todo!(),
        }
    }
    fn make_request(req: RequestBuilder, start: i64, end: i64) -> Result<f32, reqwest::Error> {
        let mut query_params: Vec<(String, String)> = Vec::new();
        query_params.push(("start_date".to_string(), format!("{}", start)));
        query_params.push(("end_date".to_string(), format!("{}", end)));
        let res = req.query(&query_params).send()?.text()?;
        let time_entries: TimeEntries = serde_json::from_str(&res).unwrap(); 
        Ok(calculate_time(time_entries))
    }

    fn format_time(time: f32) -> String {
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
}

