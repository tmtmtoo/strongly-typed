pub struct TypedValue<T, C> {
    inner: T,
    _phantom: std::marker::PhantomData<C>,
}

impl<T: PartialEq, C> PartialEq for TypedValue<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<T: Eq, C> Eq for TypedValue<T, C> {}

impl<T: PartialOrd, C> PartialOrd for TypedValue<T, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T: Ord, C> Ord for TypedValue<T, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T: Clone, C> Clone for TypedValue<T, C> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Copy, C> Copy for TypedValue<T, C> {}

impl<T: std::hash::Hash, C> std::hash::Hash for TypedValue<T, C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state)
    }
}

impl<T: std::fmt::Debug, C> std::fmt::Debug for TypedValue<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypedValue")
            .field("inner", &self.inner)
            .finish()
    }
}

impl<T: std::fmt::Display, C> std::fmt::Display for TypedValue<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<T, C> std::ops::Deref for TypedValue<T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub trait Contract {
    type Value;
    type Error;

    fn invariant(value: &Self::Value) -> Result<(), Self::Error>;
}

pub trait Encode {
    type Value;
    type Target;

    fn into(value: &Self::Value) -> Self::Target;
}

pub trait Decode {
    type Value;
    type Target;
    type Error;

    fn from(value: &Self::Target) -> Result<Self::Value, Self::Error>;
}

#[derive(Debug)]
pub enum DecodingError<D, C> {
    Decode(D),
    Contract(C),
}

impl<D: std::fmt::Display, C: std::fmt::Display> std::fmt::Display for DecodingError<D, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (kind, error) = match self {
            DecodingError::Decode(e) => ("Decode", e as &dyn std::fmt::Display),
            DecodingError::Contract(e) => ("Contract", e as &dyn std::fmt::Display),
        };

        write!(f, "Decoding failed on {} phase because: {}", kind, error)
    }
}

impl<D: std::error::Error + 'static, C: std::error::Error + 'static> std::error::Error
    for DecodingError<D, C>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        let source = match self {
            DecodingError::Decode(e) => e as &dyn std::error::Error,
            DecodingError::Contract(e) => e as &dyn std::error::Error,
        };

        Some(source)
    }
}

impl<T, C: Contract<Value = T>> TypedValue<T, C> {
    pub fn new(value: T) -> Result<Self, <C as Contract>::Error> {
        C::invariant(&value)?;

        Ok(TypedValue {
            inner: value,
            _phantom: std::marker::PhantomData,
        })
    }

    pub fn encode<E: Encode<Value = T>>(&self) -> E::Target {
        E::into(&self.inner)
    }

    pub fn decode<D: Decode<Value = T>>(
        value: &D::Target,
    ) -> Result<Self, DecodingError<D::Error, C::Error>> {
        let value = D::from(value).map_err(DecodingError::Decode)?;
        Self::new(value).map_err(DecodingError::Contract)
    }
}

pub trait TypedValueExt: Sized {
    fn typed<C: Contract<Value = Self>>(self) -> Result<TypedValue<Self, C>, C::Error>;
}

impl<T> TypedValueExt for T {
    #[inline]
    fn typed<C: Contract<Value = Self>>(self) -> Result<TypedValue<Self, C>, C::Error> {
        TypedValue::<Self, C>::new(self)
    }
}

pub mod primitive {
    use super::*;

    macro_rules! export_types {
        ( $($x:ty => $y:ident),* ) => {
            $( pub type $y<C> = TypedValue<$x, C>; )*
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

    impl<T> Contract for Stub<T> {
        type Value = T;
        type Error = ();

        fn invariant(_: &Self::Value) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    impl<T: Clone> Encode for Stub<T> {
        type Value = T;
        type Target = T;

        fn into(value: &Self::Value) -> Self::Target {
            value.clone()
        }
    }

    impl<T: Clone> Decode for Stub<T> {
        type Value = T;
        type Target = T;
        type Error = ();

        fn from(value: &Self::Target) -> Result<Self::Value, Self::Error> {
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
