//! This module contains various macros used in the rest of the codebase.

mod wrapper_enum;
mod wrapper_type;
mod xed_enum;

pub(crate) use wrapper_enum::{is_contiguous, wrapper_enum};
pub(crate) use wrapper_type::wrapper_type;
pub(crate) use xed_enum::xed_enum;

macro_rules! first {
    (
           [ $( $head:tt )* ]
        $( [ $( $rest:tt )* ] )*
    ) => {
        $( $head )*
    }
}

pub(crate) use first;
