use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough args");
        }

        let query    = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let lines = contents.lines();
    if case_sensitive {
        lines.filter(|line| { line.contains(query) }).collect()
    } else {
        lines.filter(|line| { line.to_lowercase().contains(&query.to_lowercase()) }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "lin";
        let contents = "\
so many lines
so little time
";

        assert_eq!(vec!["so many lines"], search(query, contents, true))
    }

    #[test]
    fn case_insensitive() {
        let query = "LIN";
        let contents = "\
so many LiNEs
so little time
";
        assert_eq!(vec!["so many LiNEs"], search(query, contents, false))

    }
}
