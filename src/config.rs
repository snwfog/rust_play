use std::{env, env::Args};

pub struct Config {
  pub query:          String,
  pub filename:       String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: Args) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("could not find query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("could not find filename string"),
    };

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

// pub fn search(query: &str, contents: str) -> Vec<String> {}
