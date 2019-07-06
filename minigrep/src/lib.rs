use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_be_too_few_arguments() {
        let args: Vec<String> = vec![String::from("tacos")];
        let result = Config::new(&args);
        assert!(result.is_err());
    }

    #[test]
    fn should_be_enough_arguments() {
        let args: Vec<String> = vec![
            String::from("minigrep"),
            String::from("tacos"),
            String::from("menu.txt"),
        ];
        let config = Config::new(&args);
        assert!(config.is_ok());
        let Config {
            query: q,
            filename: f,
        } = config.unwrap();
        assert_eq!("tacos", q);
        assert_eq!("menu.txt", f);
    }

    #[test]
    fn should_fail_to_find_file() {
        let config = Config {
            query: "tacos".to_string(),
            filename: "a-really-silly-filename-that-should-not-exist.txt".to_string(),
        };
        let result = run(config);
        assert!(result.is_err());
    }

    #[test]
    fn should_find_file() {
        let config = Config {
            query: "tacos".to_string(),
            filename: "Cargo.toml".to_string(),
        };
        let result = run(config);
        assert!(result.is_ok());
    }
}
