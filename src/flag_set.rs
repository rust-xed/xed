use xed_sys::*;

pub struct FlagSet(xed_flag_set_t);

impl FlagSet {
    pub fn from_ref(raw: &xed_flag_set_t) -> &Self {
        // SAFETY: SimpleFlag is #[repr(transparent)]
        unsafe { std::mem::transmute(raw) }
    }

    pub fn from_raw(raw: xed_flag_set_t) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> xed_flag_set_t {
        self.0
    }

    pub fn as_raw(&self) -> &xed_flag_set_t {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut xed_flag_set_t {
        &mut self.0
    }

    /// Get the flags as a mask.
    pub fn as_mask(&self) -> u32 {
        unsafe { xed_flag_set_mask(self.as_raw()) }
    }

    /// Whether this flag set has a subset of the flags in `other`.
    pub fn is_subset_of(&self, other: &Self) -> bool {
        unsafe { xed_flag_set_is_subset_of(self.as_raw(), other.as_raw()) != 0 }
    }
}
