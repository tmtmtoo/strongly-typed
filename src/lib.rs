pub trait Contract {
    type Value;
    type Error;

    fn invariant(value: &Self::Value) -> Result<(), Self::Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedValue<T, V> {
    inner: T,
    _phantom: std::marker::PhantomData<V>,
}

impl<T, C> TypedValue<T, C>
where
    C: Contract<Value = T>,
{
    pub fn try_new(value: T) -> Result<Self, <C as Contract>::Error> {
        C::invariant(&value)?;

        Ok(TypedValue {
            inner: value,
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<T, C> std::ops::Deref for TypedValue<T, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

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
    String  => TypedString
);
