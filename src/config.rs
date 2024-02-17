use std::{env, fs};

#[derive(Debug)]
pub struct Cfg {
    pub token: String,
    pub team_id: String,
    pub space_id: String,
    pub folder_id: String,
    pub list_id: String,
    pub daily_quota: f32,
}

impl FromIterator<(String, String)> for Cfg {
    fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
        let mut cfg = Cfg {
            token: String::new(),
            team_id: String::new(),
            space_id: String::new(),
            folder_id: String::new(),
            list_id: String::new(),
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

pub fn build_cfg() -> Cfg {
    let cfg = parse_cfg();
    let cfg: Cfg = cfg.into_iter().collect();
    if cfg.token.is_empty() || cfg.team_id.is_empty() {
        panic!("cu_auth and teamid must be set in the config file!");
    }
    cfg
}