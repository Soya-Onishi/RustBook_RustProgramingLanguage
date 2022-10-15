use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
  query: String,
  filename: String,
  case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments")
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
  
      Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = get_contents(&config.filename)?;

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

fn get_contents(filename: &str) -> Result<String, std::io::Error> {
  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  Ok(contents)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io;

const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

  #[test]
  fn get_contents_no_error() -> Result<(), io::Error> {
    let result = get_contents("./Hello.txt")?;    
    assert_eq!(
      result,
      "Hello World"
    );

    Ok(())
  }
  
  fn get_contents_with_error() {
    if let Err(err) = get_contents("./not_exist.txt") {
      assert_eq!(err.kind(), io::ErrorKind::NotFound);
    } else {
      panic!("loading ./not_exist.txt returns no error");
    }    
  }

  #[test]
  fn one_result() {
    let query = "duct";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, CONTENTS)
    )
  }

  #[test]
  fn case_sensitive() {
    let query = "Rust";

    assert_eq!(
      vec!["Rust:"],
      search(query, CONTENTS)
    )
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, CONTENTS)
    )
  }
}