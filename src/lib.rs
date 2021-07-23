pub trait Property {
    type Value;
    type Error;

    fn validate(value: &Self::Value) -> Result<(), Self::Error>;
}

pub struct TypedValue<P: Property> {
    inner: P::Value,
}

impl<P: Property> TypedValue<P> {
    pub fn new(value: P::Value) -> Result<Self, P::Error> {
        P::validate(&value)?;

        Ok(TypedValue { inner: value })
    }
}

impl<P: Property<Value = V>, V: PartialEq> PartialEq for TypedValue<P> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<P: Property<Value = V>, V: Eq> Eq for TypedValue<P> {}

impl<P: Property<Value = V>, V: PartialOrd> PartialOrd for TypedValue<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<P: Property<Value = V>, V: Ord> Ord for TypedValue<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<P: Property<Value = V>, V: Clone> Clone for TypedValue<P> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<P: Property<Value = V>, V: Copy> Copy for TypedValue<P> {}

impl<P: Property<Value = V>, V: std::fmt::Debug> std::fmt::Debug for TypedValue<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedValue")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<P: Property<Value = V>, V: std::fmt::Display> std::fmt::Display for TypedValue<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<P: Property> std::ops::Deref for TypedValue<P> {
    type Target = P::Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait TypedValueExt: Sized {
    fn typed<P: Property<Value = Self>>(self) -> Result<TypedValue<P>, P::Error>;
}

impl<T> TypedValueExt for T {
    fn typed<P: Property<Value = Self>>(self) -> Result<TypedValue<P>, P::Error> {
        TypedValue::new(self)
    }
}

#[cfg(test)]
mod property_based_tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    struct Stub<T>(T);

    impl<T> Property for Stub<T> {
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
