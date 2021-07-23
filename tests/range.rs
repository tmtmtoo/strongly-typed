extern crate typed_value;

use typed_value::*;

enum RangeProperty<const MIN: u8, const MAX: u8> {}

impl<const MIN: u8, const MAX: u8> Property for RangeProperty<MIN, MAX> {
    type Value = u8;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if (MIN..=MAX).contains(value) {
            Ok(())
        } else {
            Err(())
        }
    }
}

type ElementarySchoolGradeProperty = RangeProperty<1, 6>;

type ElementarySchoolGrade = TypedValue<ElementarySchoolGradeProperty>;

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

struct OddProperty<T>(T);

impl<T: Property<Value = u8, Error = ()>> Property for OddProperty<T> {
    type Value = u8;
    type Error = ();

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        T::validate(value)?;

        match value % 2 {
            1 => Ok(()),
            _ => Err(()),
        }
    }
}

type ElementarySchoolOddGrade = TypedValue<OddProperty<ElementarySchoolGradeProperty>>;

#[test]
fn ok_when_initialized_with_odd_grade() {
    assert!(ElementarySchoolOddGrade::new(1).is_ok())
}

#[test]
fn err_when_initialized_with_even_grade() {
    assert!(ElementarySchoolOddGrade::new(2).is_err())
}
