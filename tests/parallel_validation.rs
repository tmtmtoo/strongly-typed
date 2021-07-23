extern crate typed_value;

use typed_value::*;

const ALPHA_NUMERIC_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^[A-Za-z0-9]*$").expect("invalid regex"));

enum AlphaNumericProperty {}

impl Property for AlphaNumericProperty {
    type Value = String;
    type Error = Box<dyn std::error::Error>;

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if ALPHA_NUMERIC_REGEX.is_match(value) {
            Ok(())
        } else {
            Err(anyhow::anyhow!("\"{}\" is not alpha_numeric.", value).into())
        }
    }
}

enum FixedLengthProperty<const N: usize> {}

impl<const N: usize> Property for FixedLengthProperty<N> {
    type Value = String;
    type Error = Box<dyn std::error::Error>;

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if value.chars().count() == N {
            Ok(())
        } else {
            Err(anyhow::anyhow!("\"{}\" length is not equal to {}.", value, N).into())
        }
    }
}

struct Compose2<V, A, B>(V, A, B);

impl<V, A, B> Property for Compose2<V, A, B>
where
    A: Property<Value = V, Error = Box<dyn std::error::Error>>,
    B: Property<Value = V, Error = Box<dyn std::error::Error>>,
{
    type Value = V;
    type Error = Vec<Box<dyn std::error::Error>>;

    fn validate(value: &V) -> Result<(), Self::Error> {
        match [A::validate, B::validate]
            .iter()
            .fold(Vec::new(), |mut acc, f| match f(value) {
                Ok(_) => acc,
                Err(e) => {
                    acc.push(e);
                    acc
                }
            }) {
            errors if errors.len() == 0 => Ok(()),
            errors => Err(errors),
        }
    }
}

type FixedLengthAlphaNumericProperty<const N: usize> =
    Compose2<String, FixedLengthProperty<N>, AlphaNumericProperty>;

type FixedLengthAlphaNumeric<const N: usize> = TypedValue<FixedLengthAlphaNumericProperty<N>>;

#[test]
fn ok_when_multiple_validation() {
    let value = "1234".to_owned();
    assert!(FixedLengthAlphaNumeric::<4>::new(value).is_ok());
}

#[test]
fn multiple_errors_invalid_alphanumeric_and_invalid_length() {
    let value = "1234☺️".to_owned();
    let errors = FixedLengthAlphaNumeric::<4>::new(value).unwrap_err();
    let actual = errors.len();
    let expected = 2;
    assert_eq!(actual, expected);
}
