pub struct Config {
    pub no_color: bool,
    pub debug: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            no_color: false,
            debug: false,
        }
    }
}

const VALID_ARGS: [&str; 1] = ["--no-color"];

pub fn config(args: Vec<String>) -> Result<Config, String> {
    let mut config: Config = Default::default();
    let mut args_to_find = 0;

    for i in args.iter() {
        if args_to_find == 0 && !VALID_ARGS.contains(&i.clone().as_str()) {
            return Err(format!("Invalid argument {}", i));
        }

        if i == "--no-color" {
            config.no_color = true;
        }

        if i == "--debug" {
            config.debug = true;
        }
    }
    Ok(config)
}
