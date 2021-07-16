pub struct TypedValue<T: Validate> {
    inner: T::Value,
}

impl<T: Validate<Value = V>, V: PartialEq> PartialEq for TypedValue<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Validate<Value = V>, V: Eq> Eq for TypedValue<T> {}

impl<T: Validate<Value = V>, V: PartialOrd> PartialOrd for TypedValue<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Validate<Value = V>, V: Ord> Ord for TypedValue<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: Validate<Value = V>, V: Clone> Clone for TypedValue<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Validate<Value = V>, V: Copy> Copy for TypedValue<T> {}

impl<T: Validate<Value = V>, V: std::fmt::Debug> std::fmt::Debug for TypedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedValue")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: Validate<Value = V>, V: std::fmt::Display> std::fmt::Display for TypedValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T: Validate> std::ops::Deref for TypedValue<T> {
    type Target = T::Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Validate {
    type Value;
    type Error;

    fn validate(value: &Self::Value) -> Result<(), Self::Error>;
}

impl<T: Validate> TypedValue<T> {
    pub fn new(value: T::Value) -> Result<Self, T::Error> {
        T::validate(&value)?;

        Ok(TypedValue { inner: value })
    }
}

pub trait TypedValueExt: Sized {
    fn typed<T: Validate<Value = Self>>(self) -> Result<TypedValue<T>, T::Error>;
}

impl<T> TypedValueExt for T {
    fn typed<V: Validate<Value = Self>>(self) -> Result<TypedValue<V>, V::Error> {
        TypedValue::new(self)
    }
}

#[cfg(test)]
mod property_based_tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    struct Stub<T>(T);

    impl<T> Validate for Stub<T> {
        type Value = T;
        type Error = ();

        fn validate(_: &Self::Value) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[quickcheck]
    fn equivalent_when_wrapped_and_then_unwrapped(value: u8) {
        assert_eq!(*TypedValue::<Stub<_>>::new(value).unwrap(), value)
    }
}
