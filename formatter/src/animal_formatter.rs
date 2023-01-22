use super::{Formatter, FormatterError};
use colorizer::Colorizer;

pub struct AnimalFormatter;
impl Formatter for AnimalFormatter {
    fn format(&self, s: &str) -> Result<String, FormatterError> {
        let mut word: String = String::from("");
        for (i, l) in s.chars().enumerate() {
            word += &Colorizer.green(&format!("{}", l));
            if i < s.len() - 1 {
                word += &Colorizer.red("*")
            }
        }
        Ok(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("horse", "h*o*r*s*e")]
    #[case("giraffe", "g*i*r*a*f*f*e")]
    #[case("mouse", "m*o*u*s*e")]
    #[case("pigeon", "p*i*g*e*o*n")]
    #[case("", "")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        let actual = AnimalFormatter::format(&AnimalFormatter, input);
        assert_eq!(expected, &actual.unwrap());
    }
}
