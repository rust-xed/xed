use xed_sys::*;

use crate::raw::AsRaw;

crate::macros::wrapper_type! {
    #[derive(FromRaw, AsRaw, AsRawMut, IntoRaw)]
    pub struct FlagSet(xed_flag_set_t);
}

impl FlagSet {
    /// Get the flags as a mask.
    pub fn as_mask(&self) -> u32 {
        unsafe { xed_flag_set_mask(self.as_raw()) }
    }

    /// Whether this flag set has a subset of the flags in `other`.
    pub fn is_subset_of(&self, other: &Self) -> bool {
        unsafe { xed_flag_set_is_subset_of(self.as_raw(), other.as_raw()) != 0 }
    }
}
