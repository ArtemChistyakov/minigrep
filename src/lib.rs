use std::{env, thread};
use std::error::Error;
use std::fs;
use std::sync::{Arc, mpsc};

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    let (tx, rx) = mpsc::channel();
    let length = file_content.len();
    if length < 10000 {
        thread::spawn(|| {
            let vec = search(&config.query, &file_content);
            tx.send(vec);
        });
    } else {
        let substrs = sub_strings(&file_content, 10000);
        for substr in substrs {
            thread::spawn(move || {
                let vec = search(&config.query, substr);
                tx.send(vec);
            });
        }
    }
    for vec in rx {
        vec.iter()
            .for_each(|line| println!("{}", line));
    }
    Ok(())
}

fn sub_strings(string: &str, sub_len: usize) -> Vec<&str> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        subs.push(&string[pos..pos + len]);
        pos += len;
    }
    subs
}

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
        let query = String::from(&args[1]);
        let filename = String::from(&args[2]);
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Arc<Vec<&'a str>> {
    return Arc::new(contents
        .lines()
        .filter(|line| line.contains(query))
        .collect());
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::{search, search_case_insensitive};

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], *search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}