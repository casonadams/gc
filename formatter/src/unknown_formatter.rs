use super::{Formatter, FormatterError};

pub struct UnknownFormatter;
impl Formatter for UnknownFormatter {
    fn format(&self, s: &str) -> Result<String, FormatterError> {
        Err(FormatterError::UnknownFormatterError(s.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("chair", "Unknown word: chair")]
    #[case("", "Unknown word: ")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        let actual = UnknownFormatter::format(&UnknownFormatter, input);
        match actual {
            Ok(actual) => assert_eq!(actual, expected),
            Err(actual) => assert_eq!(actual.to_string(), expected.to_string()),
        }
    }
}
