//!

/// Helper macro used to silence `unused_import` warnings when an item is
/// only imported in order to refer to it within a doc comment.
macro_rules! used_in_docs {
    ($( $item:ident ),*) => {
        const _: () = {
            #[allow(unused_imports)]
            mod dummy {
                $( use super::$item; )*
            }
        };
    };
}

// This needs to go first so that other modules can access it.
#[macro_use]
mod macros;

pub mod error;
pub mod raw;
mod util;
