use std::fmt;

pub mod animal_formatter;
pub mod fruit_formatter;
pub mod unknown_formatter;
pub mod vegetable_formatter;

pub use animal_formatter::AnimalFormatter;
pub use fruit_formatter::FruitFormatter;
pub use unknown_formatter::UnknownFormatter;
pub use vegetable_formatter::VegetableFormatter;

#[derive(Debug, PartialEq)]
pub enum FormatterError {
    UnknownFormatterError(String),
}

impl fmt::Display for FormatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FormatterError::UnknownFormatterError(ref cause) => {
                write!(f, "Unknown word: {}", cause)
            }
        }
    }
}

pub trait Formatter {
    fn format(&self, s: &str) -> Result<String, FormatterError>;
}

pub fn format(input: &str) -> Result<String, FormatterError> {
    let words: Vec<&str> = input.split(' ').collect();
    let mut output: Vec<String> = vec![];

    for word in words {
        let clean_word: &str = &word.trim();
        let formatter: Box<dyn Formatter> = match clean_word {
            "apple" => Box::new(FruitFormatter),
            "banana" => Box::new(FruitFormatter),
            "mango" => Box::new(FruitFormatter),
            "carrot" => Box::new(VegetableFormatter),
            "zucchini" => Box::new(VegetableFormatter),
            "broccoli" => Box::new(VegetableFormatter),
            "horse" => Box::new(AnimalFormatter),
            "giraffe" => Box::new(AnimalFormatter),
            "mouse" => Box::new(AnimalFormatter),
            "pigeon" => Box::new(AnimalFormatter),
            _ => Box::new(UnknownFormatter),
        };

        output.push(formatter.format(clean_word)?);
    }
    Ok(output.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("apple banana mango\n", "APPLE BANANA MANGO")]
    #[case("carrot zucchini broccoli\n", "[carrot] [zucchini] [broccoli]")]
    #[case(
        "horse giraffe mouse pigeon\n",
        "h*o*r*s*e g*i*r*a*f*f*e m*o*u*s*e p*i*g*e*o*n"
    )]
    #[case("horse box Chair\n", "Unknown word: box")]
    fn test_format(#[case] input: &str, #[case] expected: &str) {
        match format(input) {
            Ok(actual) => assert_eq!(actual, expected),
            Err(actual) => assert_eq!(actual.to_string(), expected.to_string()),
        }
    }
}
