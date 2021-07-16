pub struct TypedValue<C: Contract> {
    inner: C::Value,
}

impl<C: Contract<Value = V>, V: PartialEq> PartialEq for TypedValue<C> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<C: Contract<Value = V>, V: Eq> Eq for TypedValue<C> {}

impl<C: Contract<Value = V>, V: PartialOrd> PartialOrd for TypedValue<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<C: Contract<Value = V>, V: Ord> Ord for TypedValue<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<C: Contract<Value = V>, V: Clone> Clone for TypedValue<C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<C: Contract<Value = V>, V: Copy> Copy for TypedValue<C> {}

impl<C: Contract<Value = V>, V: std::fmt::Debug> std::fmt::Debug for TypedValue<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedValue")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<C: Contract<Value = V>, V: std::fmt::Display> std::fmt::Display for TypedValue<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<C: Contract> std::ops::Deref for TypedValue<C> {
    type Target = C::Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Contract {
    type Value;
    type Error;

    fn invariant(value: &Self::Value) -> Result<(), Self::Error>;
}

impl<C: Contract> TypedValue<C> {
    pub fn new(value: C::Value) -> Result<Self, C::Error> {
        C::invariant(&value)?;

        Ok(TypedValue { inner: value })
    }
}

pub trait TypedValueExt: Sized {
    fn typed<C: Contract<Value = Self>>(self) -> Result<TypedValue<C>, C::Error>;
}

impl<T> TypedValueExt for T {
    fn typed<C: Contract<Value = Self>>(self) -> Result<TypedValue<C>, C::Error> {
        TypedValue::new(self)
    }
}

#[cfg(test)]
mod property_based_tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    struct Stub<T>(T);

    impl<T> Contract for Stub<T> {
        type Value = T;
        type Error = ();

        fn invariant(_: &Self::Value) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[quickcheck]
    fn equivalent_when_wrapped_and_then_unwrapped(value: u8) {
        assert_eq!(*TypedValue::<Stub<_>>::new(value).unwrap(), value)
    }
}
