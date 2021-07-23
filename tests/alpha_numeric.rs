extern crate typed_value;

use typed_value::*;

const ALPHA_NUMERIC_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^[A-Za-z0-9]+$").expect("invalid regex"));

enum AlphaNumericProperty {}

impl Property for AlphaNumericProperty {
    type Value = String;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if ALPHA_NUMERIC_REGEX.is_match(value) {
            Ok(())
        } else {
            Err(())
        }
    }
}

struct FixedLengthProperty<T, const N: usize>(T);

impl<T: Property<Value = String, Error = ()>, const N: usize> Property
    for FixedLengthProperty<T, N>
{
    type Value = String;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        T::validate(value)?;

        if value.chars().count() == N {
            Ok(())
        } else {
            Err(())
        }
    }
}

type DynamicLengthAlphaNumeric = TypedValue<AlphaNumericProperty>;

type FixedLengthAlphaNumeric<const N: usize> =
    TypedValue<FixedLengthProperty<AlphaNumericProperty, N>>;

#[test]
fn ok_when_initialized_with_alpha_numeric_string() {
    assert!(DynamicLengthAlphaNumeric::new("0123456789ABCDEF".into()).is_ok());
    assert!(FixedLengthAlphaNumeric::<16>::new("0123456789ABCDEF".into()).is_ok());
}

#[test]
fn err_when_initialized_with_alpha_numeric_string() {
    assert!(DynamicLengthAlphaNumeric::new("☺️".into()).is_err());
    assert!(FixedLengthAlphaNumeric::<1>::new("☺".into()).is_err());
}
