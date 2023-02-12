use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, &'static str> {
        let (options, not_options): (Vec<_>, Vec<_>) =
            args.partition(|arg| arg.starts_with("--"));

        let mut not_options_iter = not_options.into_iter().skip(1);
        let query = match not_options_iter.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match not_options_iter.next() {
            Some(arg) => arg,
            None => return Err("Didn't ge a file name"),
        };

        let case_sensitive =
            if options.contains(&String::from("--case-insensitive")) {
                false
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents =
            "Rust:\nsafe, fast, productive.\nPick three.\n.Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
