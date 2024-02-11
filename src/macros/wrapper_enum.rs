/// Macro for declaring an enum that wraps around an existing C enum.
macro_rules! wrapper_enum {
    {
        $( #[$attr:meta] )*
        $vis:vis enum $name:ident {
            $(
                $( #[$fattr:meta] )*
                $variant:ident = $value:expr
            ),* $(,)?
        }

        $( contiguous = $contiguous:expr; )?
    } => {
        $( #[$attr] )*
        #[repr(u8)]
        $vis enum $name {
            $(
                $( #[$fattr] )*
                $variant = $value as _,
            )*
        }

        #[automatically_derived]
        impl From<$name> for core::ffi::c_uint {
            fn from(value: $name) -> Self {
                value as _
            }
        }

        #[automatically_derived]
        impl core::convert::TryFrom<core::ffi::c_uint> for $name {
            type Error = crate::error::InvalidEnumValue;

            #[allow(non_upper_case_globals)]
            #[allow(unreachable_patterns)]
            fn try_from(value: core::ffi::c_uint) -> Result<Self, Self::Error> {
                use core::ffi::c_uint;

                $( const $variant: c_uint = $name::$variant as c_uint; )*

                const _CONTIGUOUS: Option<(c_uint, c_uint)> = $crate::macros::is_contiguous(
                    &[ $( $variant, )* ],
                    match () {
                        $( _ => $contiguous, )?
                        _ => false,
                    }
                );

                // For enums with large numbers of variants the match below
                // generates an enormous jump table that essentially boils down to
                //
                // match value {
                //   1 => 1 as enum,
                //   2 => 2 as enum,
                //   3 => 3 as enum,
                //   ...
                //   n => n as enum
                // }
                //
                // In some cases this is unavoidable. If the enum's variants are
                // not dense then we need to do the match anyway. However, many
                // XED enums _are_ dense so we can do some unsafe hacks to just
                // transmute the discriminant value into the enum. This results
                // in _much_ smaller code.
                if let Some((min, max)) = _CONTIGUOUS {
                    if !(min..=max).contains(&value) {
                        return Err(crate::error::InvalidEnumValue::new(
                            value,
                            stringify!($name)
                        ))
                    }

                    // SAFETY:
                    // There's two guarantees that we need to ensure this is safe:
                    // - We've verified that all the variants of the enum have
                    //   discriminants that fall in the range min..=max and that
                    //   there are no gaps. This was done by is_contiguous().
                    // - We've verified that value is in the range min..=max.
                    //   That was done above.
                    //
                    // Now as to whether this is fundamentally safe we refer to
                    // the unsafe code guideline reference:
                    // https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html
                    //
                    // It gives us two guarantees:
                    // - The representation of a fieldless enum is an integer is an integer of
                    //   unspecified size.
                    // - For variants with explicitly specified discriminants, the representation
                    //   of said variant is equivalent to the value of the specified discriminant.
                    //
                    // The two exceptions to this are:
                    // - Enums with zero variants. These have the same representation as the ! type.
                    // - Enums with one variant. These are ZSTs.
                    //
                    // These two exceptions are excluded by the is_contiguous() function.
                    //
                    // In all other cases it is valid to transmute an integer of the appropriate
                    // size to the enum provided that the integer value is a valid one for one of
                    // the enum's variants. We do this below.
                    Ok(match std::mem::size_of::<$name>() {
                        1 => unsafe { std::mem::transmute_copy::<_, $name>(&(value as u8))  },
                        2 => unsafe { std::mem::transmute_copy::<_, $name>(&(value as u16)) },
                        4 => unsafe { std::mem::transmute_copy::<_, $name>(&(value as u32)) },
                        8 => unsafe { std::mem::transmute_copy::<_, $name>(&(value as u64)) },
                        _ => unreachable!()
                    })
                } else {
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

        #[test]
        #[cfg(test)]
        fn test_enum_conversions() {
            use core::convert::TryFrom;

            $(
                assert_eq!(
                    $name::try_from($value)
                        .expect(concat!("failed to convert variant ", stringify!($variant))),
                    $name::$variant
                );
            )*

            assert!($name::try_from(core::ffi::c_uint::MAX).is_err());
        }
    }
}

use std::ffi::c_uint;

pub(crate) use wrapper_enum;

pub(crate) const fn is_contiguous(variants: &[c_uint], assert: bool) -> Option<(c_uint, c_uint)> {
    // There's two cases here that we care about. The hackery in wrapper_enum!
    // depends on some of the guarantees that are defined in
    // https://rust-lang.github.io/unsafe-code-guidelines/layout/enums.html
    //
    // Namely:
    // - A fieldless rust enum is represented as a simple integer of some size.
    // - The representation of the variants is equivalent to their explicitly
    //   specified discriminants.
    //
    // This means that if the variant we get is valid then we can transmute it
    // to the enum after casting it to the corresponding integer type.
    //
    // However, this doesn't apply in all cases:
    // - If the enum has no variants then it is equivalent to the ! type and so
    //   doesn't have a representation.
    // - If the enum has only 1 variant then it is a ZST so transmuting it will not
    //   be valid. In this case we should just use a normal match statement.
    //
    // This if statement checks those two conditions. Otherwise the rest of this
    // function verifies that all the variants of the enum fill in a contiguous
    // range of integers and so the transmute bounds can be verified efficiently.
    if variants.len() < 2 {
        return None;
    }

    let mut min = variants[0];
    let mut max = variants[0];
    let mut index = 0;

    while index < variants.len() {
        let variant = variants[index];

        if variant < min {
            min = variant;
        }

        if variant > max {
            max = variant;
        }

        index += 1;
    }

    let mut variant = min;
    while variant <= max {
        if !contains(variants, variant) {
            if assert {
                let slice: [(); 0] = [];

                // This statement isn't supposed to have an effect. It's just meant to create an
                // assertion message that specifies the value of the variant that is contained
                // within.
                #[allow(clippy::no_effect)]
                slice[variant as usize];
            }

            return None;
        }

        variant += 1;
    }

    Some((min, max))
}

const fn contains(variants: &[c_uint], value: c_uint) -> bool {
    let mut index = 0;

    while index < variants.len() {
        if variants[index] == value {
            return true;
        }

        index += 1;
    }

    false
}
