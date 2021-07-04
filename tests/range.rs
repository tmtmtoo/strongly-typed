extern crate strongly_typed;

use strongly_typed::{primitive::TypedU8, *};

enum RangeContract<const MIN: u8, const MAX: u8> {}

impl<const MIN: u8, const MAX: u8> Contract for RangeContract<MIN, MAX> {
    type Value = u8;
    type Error = ();

    fn invariant(value: &Self::Value) -> Result<(), Self::Error> {
        if (MIN..=MAX).contains(value) {
            Ok(())
        } else {
            Err(())
        }
    }
}

type ElementarySchoolGradeContract = RangeContract<1, 6>;

type ElementarySchoolGrade = TypedU8<ElementarySchoolGradeContract>;

#[test]
fn ok_when_initialized_with_elementary_school_grade() {
    for i in 1..=6 {
        assert!(ElementarySchoolGrade::new(i).is_ok())
    }
}

#[test]
fn err_when_initialized_with_invalid_grade() {
    assert!(ElementarySchoolGrade::new(0).is_err());
    assert!(ElementarySchoolGrade::new(7).is_err());
}

struct OddContract<T>(T);

impl<T: Contract<Value = u8, Error = ()>> Contract for OddContract<T> {
    type Value = u8;
    type Error = ();

    fn invariant(value: &Self::Value) -> Result<(), Self::Error> {
        T::invariant(value)?;

        match value % 2 {
            1 => Ok(()),
            _ => Err(()),
        }
    }
}

type ElementarySchoolOddGrade = TypedU8<OddContract<ElementarySchoolGradeContract>>;

#[test]
fn ok_when_initialized_with_odd_grade() {
    assert!(ElementarySchoolOddGrade::new(1).is_ok())
}

#[test]
fn err_when_initialized_with_even_grade() {
    assert!(ElementarySchoolOddGrade::new(2).is_err())
}
