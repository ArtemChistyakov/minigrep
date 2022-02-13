use std::{env, thread};
use std::error::Error;
use std::fs;
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(&config.filename)?;
    let result = if config.case_sensitive {
        search(&config.query, &file_content)
    } else {
        search_case_insensitive(&config.query, &file_content)
    };
    result.iter()
        .for_each(|line| println!("{}", line));
    Ok(())
}

pub fn multithreading_run(config: Config) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    let content = fs::read_to_string(&config.filename).unwrap();
    let mut vec = Vec::with_capacity(3);
    let cons = sub_strings(&content, 1000000);
    let j = cons.len();
    let query = Arc::new(config.query);
    for con in cons {
        let clone = Arc::clone(&query);
        let tx1 = tx.clone();
        let jh = thread::spawn(move || {
            let result = if config.case_sensitive {
                search_arc(clone, &con)
            } else {
                search_case_insensitive_arc(clone, &con)
            };
            let strs: Vec<String> = result.iter()
                .map(|line| line.to_string())
                .collect();
            tx1.send(strs).unwrap();
        });
        vec.push(jh);
    }
    for jh in vec {
        jh.join().unwrap();
    }
    for _ in 0..j {
        rx.recv().unwrap()
            .iter()
            .for_each(|line| println!("{}",line));
    }
    Ok(())
}

fn sub_strings(string: &str, sub_len: usize) -> Vec<String> {
    let mut subs = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }

        subs.push(string[pos..pos + len].to_string());
        pos += len;
    }
    subs
}


fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

fn search_arc(query: Arc<String>, contents: &String) -> Vec<&str> {
    return contents
        .lines()
        .filter(|line| line.contains(query.as_str()))
        .collect();
}

struct SearchResult {
    line_number:i64,
    content: String
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();
}

fn search_case_insensitive_arc(query: Arc<String>, contents: &String) -> Vec<&str> {
    let query = query.to_lowercase();
    return contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect();
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