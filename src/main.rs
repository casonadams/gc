// use formatter::format;
use colorizer::Colors;
use tokio::io::{AsyncBufReadExt, BufReader};

mod settings;
mod utils;

#[tokio::main]
async fn main() {
    let settings = settings::Settings::new().unwrap();
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
        let result = utils::multi_regex_replace_all(&line, replace_list.to_owned());
        println!("{}", result);
    }
}
