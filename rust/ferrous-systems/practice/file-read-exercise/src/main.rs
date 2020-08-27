use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use url::{ParseError, Url};

fn main() {
  let f = File::open("src/data/content.txt");

  let f = match f {
    Ok(file) => file,
    Err(err) => panic!("Error: {}", err),
  };

  let reader = BufReader::new(f);
  let lines = reader.lines();

  for line in lines {
    match line {
      Ok(string) => {
        let url = parse_url(string);
        if let Some(valid_url) = url {
          println!("{}", valid_url)
        }
      }
      Err(err) => panic!("Error: {}", err),
    };
  }
}

fn parse_url(line: String) -> Option<Url> {
  if line.is_empty() {
    None
  } else {
    let data_url = Url::parse(&line);
    match data_url {
      Ok(url) => Some(url),
      Err(_) => None,
    }
  }
}

#[test]
fn test_parse_url() -> Result<(), ParseError> {
  let issue_list_url = Url::parse("https://github.com")?;
  assert_eq!(
    parse_url("https://github.com".to_string()),
    Some(issue_list_url)
  );
  Ok(())
}

#[test]
fn test_parse_url_failure() {
  assert_eq!(parse_url("http://[:::1]".to_string()), None);
}
