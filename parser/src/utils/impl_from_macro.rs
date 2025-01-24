#[macro_export]
macro_rules! impl_from {
    ($parent_type:ident, $variant:ident) => {
        impl From<$variant> for $parent_type {
            fn from(value: $variant) -> Self {
                $parent_type::$variant(value.into())
            }
        }
    };
}
