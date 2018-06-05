use std::str::FromStr;

#[derive(Debug)]
pub struct Config {
    pub trainer_level: u8
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("not enough parameters");
        }

        let trainer_level = match u8::from_str(&args[0]) {
            Ok(lvl) if lvl >= 1 && lvl <= 40 => lvl,
            _ => return Err("trainer level must be a positive number between 1 and 40")
        };

         Ok(Config { trainer_level })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn too_low_trainer_level_test() {
        let args = vec!["0".to_string()];
        let config = Config::new(&args);

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "trainer level must be a positive number between 1 and 40");
    }

    #[test]
    fn too_high_trainer_level_test() {
        let args = vec!["145".to_string()];
        let config = Config::new(&args);

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "trainer level must be a positive number between 1 and 40");
    }

    #[test]
    fn negative_trainer_level_test() {
        let args = vec!["-1".to_string()];
        let config = Config::new(&args);

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), "trainer level must be a positive number between 1 and 40");
    }

    #[test]
    fn ok_trainer_level_test() {
        let args = vec!["35".to_string()];
        let config = Config::new(&args);

        assert!(config.is_ok());

        let config = config.unwrap();

        assert_eq!(config.trainer_level, 35);
    }
}
