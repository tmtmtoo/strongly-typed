#![no_std]
#![deny(missing_docs)]

//! A type-safe, validatable value object.

#[cfg(test)]
#[macro_use]
extern crate std;

/// Trait that represents the property of the value.
pub trait Property {
    /// Value type.
    type Value;

    /// Validation error type.
    type Error;

    /// Method for validating value.
    fn validate(value: &Self::Value) -> Result<(), Self::Error>;
}

/// Value object with constraints by Property trait.
pub struct TypedValue<P: Property> {
    inner: P::Value,
}

impl<P: Property> TypedValue<P> {
    /// Method to instantiate a TypedValue.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_value::*;
    ///
    /// struct MaximumLengthProperty<T, const N:usize>(T);
    ///
    /// impl<T: AsRef<str>, const N:usize> Property for MaximumLengthProperty<T, N> {
    ///     type Value = T;
    ///     type Error = String;
    ///
    ///     fn validate(value: &Self::Value) -> Result<(), Self::Error> {
    ///         if value.as_ref().chars().count() <= N {
    ///             Ok(())
    ///         } else {
    ///             Err(format!("length of \"{}\" is over {}.", value.as_ref(), N))
    ///         }
    ///     }
    /// }
    ///
    /// type MaximumLengthString<const N:usize> = TypedValue<MaximumLengthProperty<String, N>>;
    ///
    /// assert!(MaximumLengthString::<5>::new("foobar".to_owned()).is_err());
    /// ```
    pub fn new(value: P::Value) -> Result<Self, P::Error> {
        P::validate(&value)?;

        Ok(TypedValue { inner: value })
    }
}

impl<P: Property<Value = V>, V: PartialEq> PartialEq for TypedValue<P> {
    fn eq(&self, other: &Self) -> bool {
        V::eq(&self.inner, &other.inner)
    }
}

impl<P: Property<Value = V>, V: Eq> Eq for TypedValue<P> {}

impl<P: Property<Value = V>, V: PartialOrd> PartialOrd for TypedValue<P> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        V::partial_cmp(&self.inner, &other.inner)
    }

    fn lt(&self, other: &Self) -> bool {
        V::lt(&self.inner, &other.inner)
    }

    fn le(&self, other: &Self) -> bool {
        V::le(&self.inner, &other.inner)
    }

    fn gt(&self, other: &Self) -> bool {
        V::gt(&self.inner, &other.inner)
    }

    fn ge(&self, other: &Self) -> bool {
        V::ge(&self.inner, &other.inner)
    }
}

impl<P: Property<Value = V>, V: Ord> Ord for TypedValue<P> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        V::cmp(&self.inner, &other.inner)
    }
}

impl<P: Property<Value = V>, V: core::hash::Hash> core::hash::Hash for TypedValue<P> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        V::hash(&self.inner, state);
    }
}

impl<P: Property<Value = V>, V: Clone> Clone for TypedValue<P> {
    fn clone(&self) -> Self {
        Self {
            inner: V::clone(&self.inner),
        }
    }
}

impl<P: Property<Value = V>, V: Copy> Copy for TypedValue<P> {}

impl<P: Property<Value = V>, V: core::fmt::Debug> core::fmt::Debug for TypedValue<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        V::fmt(&self.inner, f)
    }
}

impl<P: Property<Value = V>, V: core::fmt::Display> core::fmt::Display for TypedValue<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        V::fmt(&self.inner, f)
    }
}

impl<P: Property> core::ops::Deref for TypedValue<P> {
    type Target = P::Value;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Trait that provides a wrapping method to TypedValue for any value.
pub trait TypedValueExt: Sized {
    /// # Examples
    ///
    /// ```
    /// use typed_value::*;
    ///
    /// struct MaximumLengthProperty<T, const N:usize>(T);
    ///
    /// impl<T: AsRef<str>, const N:usize> Property for MaximumLengthProperty<T, N> {
    ///     type Value = T;
    ///     type Error = String;
    ///
    ///     fn validate(value: &Self::Value) -> Result<(), Self::Error> {
    ///         if value.as_ref().chars().count() <= N {
    ///             Ok(())
    ///         } else {
    ///             Err(format!("length of \"{}\" is over {}.", value.as_ref(), N))
    ///         }
    ///     }
    /// }
    ///
    /// type MaximumLengthString<const N:usize> = TypedValue<MaximumLengthProperty<String, N>>;
    ///
    /// let foobar = "foobar".to_owned();
    /// let _: MaximumLengthString<6> = foobar.typed().unwrap();
    /// ```
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
