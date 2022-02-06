use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", file_content);
    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_config_test() {}
}