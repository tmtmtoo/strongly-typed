trait Validate {
    type Value;
    type Error;

    fn validate(value: &Self::Value) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
struct TypedNumber<T, U> {
    inner: T,
    _phantom: std::marker::PhantomData<U>,
}

impl<T, U> TypedNumber<T, U>
where
    T: Copy + std::ops::Add<Output = T>,
    U: Validate<Value = T>,
{
    fn try_new(value: T) -> Result<Self, <U as Validate>::Error> {
        U::validate(&value)?;

        Ok(TypedNumber {
            inner: value,
            _phantom: std::marker::PhantomData,
        })
    }

    fn add(&self, other: &Self) -> Result<Self, <U as Validate>::Error> {
        Self::try_new(self.inner + other.inner)
    }
}

impl<T, U> std::ops::Deref for TypedNumber<T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
struct UpperLimitedUIntValidation<T, const N: u64> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T, const N: u64> Validate for UpperLimitedUIntValidation<T, N>
where
    T: Into<u64> + Copy + std::fmt::Debug + std::fmt::Display,
{
    type Value = T;
    type Error = String;

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        if Into::<u64>::into(*value) <= N {
            Ok(())
        } else {
            Err(format!("{} is over {}", value, N))
        }
    }
}

type UpperLimitedUInt<T, const N: u64> = TypedNumber<T, UpperLimitedUIntValidation<T, N>>;

impl<T, const N: u64> UpperLimitedUInt<T, N>
where
    T: std::convert::TryFrom<u64>,
{
    fn max(&self) -> T {
        match T::try_from(N) {
            Ok(n) => n,
            Err(_) => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
struct RangeLimitedUIntValidation<T, const N: u64, const M: u64> {
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Debug)]
enum OutOfRange {
    Higher,
    Lower,
}

impl<T, const N: u64, const M: u64> Validate for RangeLimitedUIntValidation<T, N, M>
where
    T: Into<u64> + Copy,
{
    type Value = T;
    type Error = OutOfRange;

    fn validate(value: &Self::Value) -> Result<(), Self::Error> {
        match Into::<u64>::into(*value) {
            v if v < N => Err(OutOfRange::Lower),
            v if v > M => Err(OutOfRange::Higher),
            _ => Ok(()),
        }
    }
}

type RangeLimitedUInt<T, const N: u64, const M: u64> =
    TypedNumber<T, RangeLimitedUIntValidation<T, N, M>>;

type JuniorSchoolGrade = RangeLimitedUInt<u8, 1, 6>;

impl JuniorSchoolGrade {
    fn next_grade(&self) -> Result<Self, String> {
        Self::try_new(self.inner + 1).map_err(|e| match e {
            OutOfRange::Higher => "Graduated".to_owned(),
            OutOfRange::Lower => unreachable!(),
        })
    }
}

fn main() {
    unimplemented!()
}
