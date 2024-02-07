/// Macro for declaring an enum that wraps around an existing C enum.
macro_rules! wrapper_enum {
    {
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident : $ty:ty {
            $(
                $( #[$fattr:meta] )*
                $variant:ident = $value:expr
            ),* $(,)?
        }
    } => {
        $( #[$attr] )*
        // Note that I explicitly haven't added #[repr($ty)] here. It makes
        // the enums bigger for no real reason and the conversions are enough
        // to make things work.
        $vis enum $name {
            $(
                $( #[$fattr] )*
                $variant = $value as _,
            )*
        }

        #[automatically_derived]
        impl From<$name> for $ty {
            fn from(value: $name) -> Self {
                value as _
            }
        }

        #[automatically_derived]
        impl core::convert::TryFrom<$ty> for $name {
            type Error = crate::error::InvalidEnumValue<$ty>;

            #[allow(non_upper_case_globals)]
            fn try_from(value: $ty) -> Result<Self, Self::Error> {
                $( const $variant: $ty = $name::$variant as $ty; )*

                match value {
                    $( $variant => Ok(Self::$variant), )*
                    _ => Err(crate::error::InvalidEnumValue::new(
                        value,
                        stringify!($name)
                    ))
                }
            }
        }
    }
}

pub(crate) use wrapper_enum;
