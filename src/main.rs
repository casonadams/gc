use colorizer::{Colorizer, Colors};
// use formatter::format;
use regex::Regex;
use tokio::io::{AsyncBufReadExt, BufReader};

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Highlight {
    regex: String,
    name: String,
    color: Colors,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    highlight: Vec<Highlight>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let _run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("Settings"))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}

fn multi_regex_replace_all(s: &str, replace_list: Vec<(String, String, Colors)>) -> String {
    let mut result = s.to_string();
    for (pattern, replacement, color) in replace_list {
        let re = Regex::new(&pattern).unwrap();
        match color {
            Colors::RED => {
                result = re
                    .replace_all(&result, Colorizer.red(&replacement))
                    .to_string()
            }
            Colors::GREEN => {
                result = re
                    .replace_all(&result, Colorizer.green(&replacement))
                    .to_string()
            }
            Colors::YELLOW => {
                result = re
                    .replace_all(&result, Colorizer.yellow(&replacement))
                    .to_string()
            }
            Colors::BLUE => {
                result = re
                    .replace_all(&result, Colorizer.blue(&replacement))
                    .to_string()
            }
            Colors::MAGENTA => {
                result = re
                    .replace_all(&result, Colorizer.magenta(&replacement))
                    .to_string()
            }
            Colors::CYAN => {
                result = re
                    .replace_all(&result, Colorizer.cyan(&replacement))
                    .to_string()
            }
            _ => {
                result = re
                    .replace_all(&result, &replacement)
                    .to_string()
            }
        }
    }
    result
}

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();
    let mut replace_list: Vec<(String, String, Colors)> = vec![];
    for setting in settings.highlight {
        replace_list.push((
            format!(r"(?-u:\b)(?P<{}>{})", setting.name, setting.regex),
            format!("${}", setting.name),
            setting.color,
        ))
    }

    let reader = BufReader::new(tokio::io::stdin());
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        let result = multi_regex_replace_all(&line, replace_list.to_owned());
        println!("{}", result);
    }
}
