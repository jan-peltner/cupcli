use std::{env, fs};
pub mod cfg {
    use super::*;
    #[derive(Debug)]
    pub struct Cfg {
        pub teamid: String,
        pub token: String,
    }

    impl FromIterator<(String, String)> for Cfg {
        fn from_iter<I: IntoIterator<Item = (String, String)>>(iter: I) -> Self {
            let mut cfg = Cfg {
                teamid: String::new(),
                token: String::new(),
            };
            for (key, value) in iter {
                match key.as_str() {
                    "teamid" => cfg.teamid = value,
                    "cu_auth" => cfg.token = value,
                    _ => panic!("Invalid key in config file!"),
                }
            }
            cfg
        }
    }

    fn get_args() -> Vec<String> {
        let args: Vec<String> = std::env::args().collect();
        args
    }

    pub fn parse_cfg() -> Cfg {
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
        let cfg: Cfg = cfg_str
            .lines()
            .map(|line| {
                let mut split = line.split("=");
                let key = split.next().unwrap().to_string();
                let value = split.next().unwrap().to_string();
                (key, value)
            })
            .collect();

        dbg!(&cfg);
        cfg
    }

    pub fn parse_args() {
        let mut args = get_args();
        if args.len() != 2 {
            panic!("Cupcli expects exactly one argument!")
        }
        match args.pop().unwrap().to_lowercase().as_str() {
            "workspace" => println!("workspace"),
            _ => print!("yo"),
        }
    }
}
