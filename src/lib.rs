use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Config) -> bool {
        self.query == other.query && self.filename == other.filename
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_new_config() {
        let args = valid_args();
        assert_eq!(Config::new(&args).unwrap(), Config { query: "two".to_string(), filename: "poem.txt".to_string() })
    }
    #[test]
    fn create_invalid_config() {
        let args = invalid_args_len();
        assert!(Config::new(&args).is_err(), "not enough arguments")
    }

    #[test]
    fn valid_run() {
        let args = valid_args();
        let config = Config::new(&args).unwrap();
        if let Err(e) = run(config) {
            panic!("Application error: {}", e);
        }
        assert!(true)
    }

    #[test]
    fn invalid_run() {
        let args = invalid_args_filename();
        let config = Config::new(&args).unwrap();
        assert!(run(config).is_err())
    }

    fn valid_args() -> Vec<String> {
        vec!("one".to_string(), "two".to_string(), "poem.txt".to_string())
    }

    fn invalid_args_len() -> Vec<String> {
        vec!("one".to_string(), "two".to_string())
    }

    fn invalid_args_filename() -> Vec<String> {
        vec!("one".to_string(), "two".to_string(), "fake.txt".to_string())
    }
}