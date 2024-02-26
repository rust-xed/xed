// FIXME: move to a procedural macros
macro_rules! wrapper_type {
    (
        $( #[doc = $comment:literal] )?
        #[derive ( $( $trait:ident ),* ) ]
        $vis:vis struct $name:ident ( $ctype:ident );
    ) => {
        $( #[doc = $comment] )?
        #[repr(transparent)]
        $vis struct $name( $ctype );

        crate::macros::wrapper_type! (__impl $name $ctype $( $trait )* );
    };

    (
        __impl $name:ident $ctype:ident FromRaw $( $rest:ident )*
    ) => {
        impl crate::raw::FromRaw for $name {
            type CType = $ctype;

            fn from_raw(raw: Self::CType) -> Self {
                $name ( raw )
            }

            fn from_ref(raw: &Self::CType) -> &Self {
                unsafe { std::mem::transmute(raw) }
            }
        }

        crate::macros::wrapper_type! (__impl $name $ctype $( $rest )* );
    };

    (
        __impl $name:ident $ctype:ident AsRaw $( $rest:ident )*
    ) => {
        impl crate::raw::AsRaw for $name {
            fn as_raw(&self) -> &Self::CType {
                &self.0
            }
        }

        crate::macros::wrapper_type! (__impl $name $ctype $( $rest )* );
    };

    (
        __impl $name:ident $ctype:ident AsRawMut $( $rest:ident )*
    ) => {
        impl crate::raw::AsRawMut for $name {
            fn as_raw_mut(&mut self) -> &mut Self::CType {
                &mut self.0
            }
        }

        crate::macros::wrapper_type! (__impl $name $ctype $( $rest )* );
    };

    (
        __impl $name:ident $ctype:ident IntoRaw $( $rest:ident )*
    ) => {
        impl crate::raw::IntoRaw for $name {
            fn into_raw(self) -> Self::CType {
                self.0
            }
        }

        crate::macros::wrapper_type! (__impl $name $ctype $( $rest )* );
    };

    (
        __impl $name:ident $ctype:ident
    ) => {
    };
}

pub(crate) use wrapper_type;
