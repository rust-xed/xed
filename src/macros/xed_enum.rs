macro_rules! xed_enum {
    {
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident => $base:ident {
            $(
                $( #[$vattr:meta] )*
                $variant:ident $( => $value:ident )?
            ),* $(,)?
        }

        $( name = $enum_name:ident ; )?
        $( invalid = $invalid:ident; )?
    } => {
        $( #[$attr] )*
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        $vis struct $name(std::num::NonZeroU32);

        impl $name {
            const fn build_variant(value: u32, name: &str) -> Self {
                match std::num::NonZeroU32::new(value) {
                    Some(value) => Self(value),
                    None => panic!("{}", name),
                }
            }

            paste::paste! {
                $(
                    $( #[$vattr] )*
                    pub const $variant: Self = Self::build_variant(
                        $crate::macros::first!(
                            $( [ $value ] )?
                            [ [<$base _ $variant>] ]
                        ),
                        stringify!($variant)
                    );
                )*
            }
        }

        impl $name {
            #[doc = concat!(
                "Create a `", stringify!($name), "` from the underlying enum value."
            )]
            pub const fn from_raw(value: u32) -> Option<Self> {
                match std::num::NonZeroU32::new(value) {
                    Some(value) => Some(Self(value)),
                    None => None
                }
            }

            /// Convert this value into the underlying enum value.
            pub const fn into_raw(self) -> u32 {
                self.0.get()
            }
        }

        impl From<$name> for u32 {
            fn from(value: $name) -> u32 {
                value.into_raw()
            }
        }

        impl TryFrom<u32> for $name {
            type Error = crate::error::InvalidEnumValue;

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                match Self::from_raw(value) {
                    Some(value) => Ok(value),
                    None => Err(crate::error::InvalidEnumValue::new(
                        value,
                        stringify!($name)
                    ))
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let value = self.into_raw();
                let cstr = unsafe {
                    paste::paste!($crate::macros::first!(
                        $( [ xed_sys::[< $enum_name 2str >](value) ] )?
                        [ xed_sys::[< $base:lower _enum_t2str >](value) ]
                    ))
                };

                if cstr.is_null() {
                    return write!(f, "{}", value);
                }

                let cstr = unsafe { std::ffi::CStr::from_ptr(cstr) };
                write!(f, "{}", crate::util::DisplayCStr(cstr))
            }
        }

        paste::paste! {
            #[test]
            fn [< $name:snake:lower _up_to_date >]() {
                let variants: &[$name] = &[
                    $( $name::$variant, )*
                ];

                let max = variants
                    .iter()
                    .copied()
                    .map(|variant| variant.into_raw())
                    .max();

                match max {
                    Some(max) => assert_eq!(
                        max + 1,
                        [< $base _ LAST >],
                        "Enum definition not up to date: last enum value {} is not one less than {}",
                        $name::from_raw(max).unwrap(),
                        stringify!([< $base _ LAST >])
                    ),
                    None => assert_eq!(
                        1 + $crate::macros::first!(
                            $( [ $invalid ] )?
                            [ [< $base _ INVALID >] ]
                        ),
                        [< $base _ LAST >],
                        "Enum definition not up to date: enum has no variants but {} is not 1",
                        stringify!([< $base _ LAST >])
                    )
                }
            }
        }
    };

    // Variant that derives the enum stem from the rust enum name
    {
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident {
            $(
                $( #[$vattr:meta] )*
                $variant:ident $( => $value:ident )?
            ),* $(,)?
        }

        $( name = $enum_name:ident; )?
        $( invalid = $invalid:ident; )?
    } => {
        paste::paste! {
            $crate::macros::xed_enum! {
                $( #[$attr] )*
                $vis enum $name => [< XED _ $name:snake:upper >] {
                    $(
                        $( #[$vattr] )*
                        $variant $( => $value )?,
                    )*
                }

                $( name = $enum_name; )?
                $( invalid = $invalid; )?
            }
        }
    }
}

pub(crate) use xed_enum;
