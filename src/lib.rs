pub struct TypedValue<T, V> {
    inner: T,
    _phantom: std::marker::PhantomData<V>,
}

impl<T: PartialEq, V> PartialEq for TypedValue<T, V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Eq, V> Eq for TypedValue<T, V> {}

impl<T: PartialOrd, V> PartialOrd for TypedValue<T, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Ord, V> Ord for TypedValue<T, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: Clone, V> Clone for TypedValue<T, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Copy, V> Copy for TypedValue<T, V> {}

impl<T: std::hash::Hash, V> std::hash::Hash for TypedValue<T, V> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T: std::fmt::Debug, V> std::fmt::Debug for TypedValue<T, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedValue")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: std::fmt::Display, V> std::fmt::Display for TypedValue<T, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T, V> std::ops::Deref for TypedValue<T, V> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Validate {
    type Value;
    type Error;

    fn validate(value: &Self::Value) -> Result<(), Self::Error>;
}

pub trait Encode {
    type Value;
    type Target;

    fn encode(value: &Self::Value) -> Self::Target;
}

pub trait Decode {
    type Value;
    type Target;
    type Error;

    fn decode(value: &Self::Target) -> Result<Self::Value, Self::Error>;
}

#[derive(Debug)]
pub enum DecodingError<D, V> {
    Decode(D),
    Validate(V),
}

impl<D: std::fmt::Display, V: std::fmt::Display> std::fmt::Display for DecodingError<D, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, error) = match self {
            DecodingError::Decode(e) => ("Decode", e as &dyn std::fmt::Display),
            DecodingError::Validate(e) => ("Validate", e as &dyn std::fmt::Display),
        };

        write!(f, "Decoding failed on {} phase because: {}", kind, error)
    }
}

impl<D: std::error::Error + 'static, V: std::error::Error + 'static> std::error::Error
    for DecodingError<D, V>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let source = match self {
            DecodingError::Decode(e) => e as &dyn std::error::Error,
            DecodingError::Validate(e) => e as &dyn std::error::Error,
        };

        Some(source)
    }
}

impl<T, V: Validate<Value = T>> TypedValue<T, V> {
    pub fn new(value: T) -> Result<Self, <V as Validate>::Error> {
        V::validate(&value)?;

        Ok(TypedValue {
            inner: value,
            _phantom: std::marker::PhantomData,
        })
    }

    pub fn encode<E: Encode<Value = T>>(&self) -> E::Target {
        E::encode(&self.inner)
    }

    pub fn decode<D: Decode<Value = T>>(
        value: &D::Target,
    ) -> Result<Self, DecodingError<D::Error, V::Error>> {
        let value = D::decode(value).map_err(DecodingError::Decode)?;
        Self::new(value).map_err(DecodingError::Validate)
    }
}

pub trait TypedValueExt: Sized {
    fn typed<V: Validate<Value = Self>>(self) -> Result<TypedValue<Self, V>, V::Error>;
}

impl<T> TypedValueExt for T {
    #[inline]
    fn typed<V: Validate<Value = Self>>(self) -> Result<TypedValue<Self, V>, V::Error> {
        TypedValue::<Self, V>::new(self)
    }
}

pub mod primitive {
    use super::*;

    macro_rules! export_types {
        ( $($x:ty => $y:ident),* ) => {
            $( pub type $y<V> = TypedValue<$x, V>; )*
        };
    }

    export_types!(
        u8      => TypedU8,
        u16     => TypedU16,
        u32     => TypedU32,
        u64     => TypedU64,
        u128    => TypedU128,
        usize   => TypedUSize,
        i8      => TypedI8,
        i16     => TypedI16,
        i32     => TypedI32,
        i64     => TypedI64,
        i128    => TypedI128,
        usize   => TypedISize,
        f32     => TypedF32,
        f64     => TypedF64,
        char    => TypedChar,
        String  => TypedString
    );
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

    impl<T: Clone> Encode for Stub<T> {
        type Value = T;
        type Target = T;

        fn encode(value: &Self::Value) -> Self::Target {
            value.clone()
        }
    }

    impl<T: Clone> Decode for Stub<T> {
        type Value = T;
        type Target = T;
        type Error = ();

        fn decode(value: &Self::Target) -> Result<Self::Value, Self::Error> {
            Ok(value.clone())
        }
    }

    #[quickcheck]
    fn equivalent_when_wrapped_and_then_unwrapped(value: u8) {
        assert_eq!(*TypedValue::<_, Stub<_>>::new(value).unwrap(), value)
    }

    #[quickcheck]
    fn equivalent_when_encode_to_the_same_value(value: u8) {
        assert_eq!(
            TypedValue::<_, Stub<_>>::new(value)
                .unwrap()
                .encode::<Stub<_>>(),
            value
        )
    }

    #[quickcheck]
    fn equivalent_when_decode_from_the_same_value(value: u8) {
        assert_eq!(
            *TypedValue::<_, Stub<_>>::decode::<Stub<_>>(&value).unwrap(),
            value
        )
    }
}
