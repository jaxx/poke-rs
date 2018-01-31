use std::str::FromStr;

pub struct Config {
    pub trainer_level: u8
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough parameters");
        }

        let trainer_level = match u8::from_str(&args[1]) {
            Ok(lvl) => lvl,
            Err(_) => return Err("trainer level must be a positive number")
        };

        if trainer_level > 40 {
            return Err("trainer level must be smaller than 41");
        }

         Ok(Config {
            trainer_level: trainer_level
        })
    }
}
