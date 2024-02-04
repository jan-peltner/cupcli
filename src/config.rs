use std::{env, fs};
#[derive(Debug)]
pub struct Cfg {
    pub token: String,
    pub team_id: String,
    pub space_id: String,
    pub folder_id: String,
    pub list_id: String,
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
        let mut cfg = Cfg {
            token: String::new(),
            team_id: String::new(),
            space_id: String::new(),
            folder_id: String::new(),
            list_id: String::new(),
            arg: (Mode::TimeGet, String::new()),
            daily_quota: 8.0,
        };
        for (key, value) in iter {
            match key.as_str() {
                "cu_auth" => cfg.token = value,
                "teamid" => cfg.team_id = value,
                "spaceid" => cfg.space_id = value,
                "folderid" => cfg.folder_id = value,
                "listid" => cfg.list_id = value,
                "dailyQuota" => cfg.daily_quota = value.parse::<f32>().unwrap_or(8.0),
                "timeget" => cfg.arg = (Mode::TimeGet, value),
                "timetrack" => cfg.arg = (Mode::TimeTrack, value),
                _ => println!("[WARNING] Ignoring unknown key in cfg `{}`", key)
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
    let cfg: Cfg = cfg.into_iter().collect();
    if cfg.token.is_empty() || cfg.team_id.is_empty() {
        panic!("cu_auth and teamid must be set in the config file!");
    }
    cfg
}