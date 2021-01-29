use std::{error::Error, fs};
extern crate clap;
use clap::{App, Arg};

pub fn init_clap() -> Vec<String> {
  let matches = App::new("Minigrep")
    .version("1.0.1")
    .author("Jesse Parsons <savvyjesse@aol.com>")
    .about("Lightweight grep utility")
    .arg(
      Arg::with_name("insensitive")
        .short("i")
        .long("insensitive")
        .help("Sets case insensitivity"),
    )
    .arg(
      Arg::with_name("line numbers")
        .short("l")
        .long("line-numbers")
        .help("Shows line numbers for queried pattern"),
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
    matches.is_present("line numbers").to_string(),
  ]
}
pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
  pub line_numbers: bool,
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
      &_ => return Err("problem with argument parsing"),
    };
    let line_numbers = match args[3].as_str() {
      "false" => false,
      "true" => true,
      _ => return Err("problem with argument parsing"),
    };

    Ok(Config {
      query,
      filename,
      case_sensitive,
      line_numbers,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents, config.line_numbers)
  } else {
    search_case_insensitive(&config.query, &contents, config.line_numbers)
  };

  for line in results {
    println!("{}", line);
  }
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, line_numbers: bool) -> Vec<String> {
  // conditionally set line_num if config.line_numbers is true.
  // if so, add line numbers to results
  let mut results = Vec::new();

  if line_numbers {
    let mut ind = 1;
    for line in contents.lines() {
      if line.contains(query) {
        results.push(String::from(format!("{} {}", ind, line)))
      }
      ind += 1;
    }
  } else {
    for line in contents.lines() {
      if line.contains(query) {
        results.push(line.to_string());
      }
    }
  }

  results
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
  line_numbers: bool,
) -> Vec<String> {
  let mut results = Vec::new();
  if line_numbers {
    let mut ind = 1;
    for line in contents.lines() {
      if line.to_lowercase().contains(&query.to_lowercase()) {
        results.push(String::from(format!("{} {}", ind, line)))
      }
      ind += 1;
    }
  } else {
    for line in contents.lines() {
      if line.to_lowercase().contains(&query.to_lowercase()) {
        results.push(line.to_string())
      }
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
    let line_numbers = false;

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents, line_numbers)
    );
  }
  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    let line_numbers = false;
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents, line_numbers)
    );
  }
}
