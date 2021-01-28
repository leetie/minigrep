use std::{error::Error, fs};
extern crate clap;
use clap::{App, Arg};

pub fn init_clap() -> Vec<String> {
  let matches = App::new("Minigrep")
    .version("1.0")
    .author("Jesse Parsons <savvyjesse@aol.com>")
    .about("Lightweight grep utility")
    .arg(
      Arg::with_name("insensitive")
        .short("i")
        .long("insensitive")
        .help("Sets case insensitivity"),
    )
    .arg(
      Arg::with_name("QUERY")
        .help("Sets the query string")
        .required(true)
        .index(1),
    )
    .arg(
      Arg::with_name("FILENAME")
        .help("The file to run the query against")
        .required(true)
        .index(2),
    )
    .get_matches();

  vec![
    matches.value_of("QUERY").unwrap().to_string(),
    matches.value_of("FILENAME").unwrap().to_string(),
    matches.is_present("insensitive").to_string(),
  ]
}
pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("not enough arguments");
    }
    let query = args[0].clone();
    let filename = args[1].clone();
    let case_sensitive = match args[2].as_str() {
      "false" => true,
      "true" => false,
      &_ => panic!("big ow"),
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
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query.to_lowercase()) {
      results.push(line)
    }
  }

  results
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct";

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
