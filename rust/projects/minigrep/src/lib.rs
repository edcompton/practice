use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text {}", contents);
    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_new_config() {
        assert_eq!(
            Config::new(&["test".into(), "test arg 1".into(), "test arg 2".into()]).unwrap(),
            Config {
                query: String::from("test arg 1"),
                filename: String::from("test arg 2"),
            }
        );
    }

    #[test]
    fn error_config_create() {
        let result = Config::new(&["test arg 1".into(), "test arg 2".into()])
            .expect_err("not enough arguments!");
        assert_eq!(result, "not enough arguments!")
    }
}
