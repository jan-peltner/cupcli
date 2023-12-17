use std::{env, fs};
pub mod cfg {
    use super::*;

    #[derive(Debug)]
    pub struct Cfg {
        pub teamid: String,
        pub token: String,
        pub arg: (Mode, String),
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
            };
            for (key, value) in iter {
                match key.as_str() {
                    "teamid" => cfg.teamid = value,
                    "cu_auth" => cfg.token = value,
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
