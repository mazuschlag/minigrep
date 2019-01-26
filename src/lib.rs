use std::fs;
use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[derive(Debug)]
pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename , case_sensitive})
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
        assert_eq!(
            Config::new(&args).unwrap(), 
            Config { 
                query: "two".to_string(), 
                filename: "poem.txt".to_string(), 
                case_sensitive: false 
            }
        );
    }
    #[test]
    fn create_invalid_config() {
        let args = invalid_args_len();
        assert!(
            Config::new(&args).is_err(), 
            "not enough arguments"
        );
    }

    #[test]
    fn valid_run() {
        let args = valid_args();
        let config = Config::new(&args).unwrap();
        if let Err(e) = run(config) {
            panic!("Application error: {}", e);
        }
        assert!(true);
    }

    #[test]
    fn invalid_run() {
        let args = invalid_args_filename();
        let config = Config::new(&args).unwrap();
        assert!(run(config).is_err());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nRustles my jimmies!";
        assert_eq!(
            vec!["Rust:", "Rustles my jimmies!"],
            search_case_insensitive(query, contents)
        );
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