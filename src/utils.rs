use regex::Regex;
use colorizer::{Colorizer, Colors};

pub fn multi_regex_replace_all(s: &str, replace_list: Vec<(String, String, Colors)>) -> String {
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
            _ => result = re.replace_all(&result, &replacement).to_string(),
        }
    }
    result
}
