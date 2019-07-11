use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = if args.len() >= 4 {
            args[3] != "y"
        } else {
            env::var("CASE_INSENSITIVE").is_err()
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
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
            String::from("nah"),
        ];
        let config = Config::new(&args);
        assert!(config.is_ok());
        let Config {
            query: q,
            filename: f,
            case_sensitive: b,
        } = config.unwrap();
        assert_eq!("tacos", q);
        assert_eq!("menu.txt", f);
        assert_eq!(true, b);
    }

    #[test]
    fn should_fail_to_find_file() {
        let config = Config {
            query: "tacos".to_string(),
            filename: "a-really-silly-filename-that-should-not-exist.txt".to_string(),
            case_sensitive: false,
        };
        let result = run(config);
        assert!(result.is_err());
    }

    #[test]
    fn should_find_file() {
        let config = Config {
            query: "tacos".to_string(),
            filename: "Cargo.toml".to_string(),
            case_sensitive: true,
        };
        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
