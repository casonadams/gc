pub struct Colorizer;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Colors {
    RED,
    GREEN,
    YELLOW,
    BLUE,
    MAGENTA,
    CYAN,
    WHITE,
}

const RESET: &str = "\x1b[0m";
// const BOLD: &str = "\x1b[1m";
// const UNDERLINE: &str = "\x1b[7m";
// const INVERSE: &str = "\x1b[3m";
// const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const WHITE: &str = "\x1b[37m";

impl Colorizer {
    pub fn red(&self, msg: &str) -> String {
        format!("{}{}{}", RED, msg, RESET)
    }
    pub fn green(&self, msg: &str) -> String {
        format!("{}{}{}", GREEN, msg, RESET)
    }
    pub fn yellow(&self, msg: &str) -> String {
        format!("{}{}{}", YELLOW, msg, RESET)
    }
    pub fn blue(&self, msg: &str) -> String {
        format!("{}{}{}", BLUE, msg, RESET)
    }
    pub fn magenta(&self, msg: &str) -> String {
        format!("{}{}{}", MAGENTA, msg, RESET)
    }
    pub fn cyan(&self, msg: &str) -> String {
        format!("{}{}{}", CYAN, msg, RESET)
    }
    pub fn white(&self, msg: &str) -> String {
        format!("{}{}{}", WHITE, msg, RESET)
    }
}
