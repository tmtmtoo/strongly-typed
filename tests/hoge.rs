extern crate typed_value;

use typed_value::*;

enum MaxValidation<const N: u8> {}

impl<const N: u8> Validate for MaxValidation<N> {
    type Value = u8;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if value <= &N {
            Ok(())
        } else {
            Err(())
        }
    }
}

enum MinValidation<const N: u8> {}

impl<const N: u8> Validate for MinValidation<N> {
    type Value = u8;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if value > &N {
            Ok(())
        } else {
            Err(())
        }
    }
}

struct RangeValidation<A, B>(A, B);
