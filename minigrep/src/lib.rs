use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::env;

#[derive(
    Debug,
    PartialEq
)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];

        Ok(Config{
            query: query.to_string(),
            filename: filename.to_string(),
            case_sensitive: env::var("CASE_SENSITIVE").is_err()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

     let mut contents = String::new();

    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}


fn search_case_sensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in haystack.lines() {
        if line.contains(needle) {
            res.push(line);
        }
    }
    res
}

fn search_case_insensitive<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in haystack.lines() {
        if line.to_lowercase().contains(needle) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_creation_no_args() {
        let res = match Config::new(&vec![]) {
            Err(err) => err,
            Ok(_) => "Got config"
        };
        assert_eq!(
            "Not enough arguments",
            res
        );
    }

    #[test]
    fn config_creation_not_enough_args() {
        let res = match Config::new(&vec!["badabing".to_string()]) {
            Err(err) => err,
            Ok(_) => "Got config"
        };
        assert_eq!(
            "Not enough arguments",
            res
        );
    }
    
    #[test]
    fn config_creation_enough_args() {
        assert_eq!(
            Ok(Config{query: "badabing".to_string(), filename: "badaboom".to_string(), case_sensitive: false }),
            Config::new(&vec!["boop".to_string(), "badabing".to_string(), "badaboom".to_string()])
        );
    }

    #[test]
    fn missing_file() {
        assert!(
            run(Config::new(&vec![
                "boop".to_string(),
                "badabing".to_string(),
                "badaboom".to_string()
            ]).unwrap()).is_err()
        );
    }

    #[test]
    fn search_case_sensitive_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
        ";

        assert_eq!(
            vec!["safe, fast, productive."], 
            search_case_sensitive(query, contents)
        );
    }
}