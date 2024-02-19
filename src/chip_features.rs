use xed_sys::*;

use super::{
    raw::{AsRawMut, IntoRaw},
    Chip, IsaSet,
};

crate::macros::wrapper_type! {
    #[derive(FromRaw, AsRaw, AsRawMut, IntoRaw)]
    pub struct ChipFeatures(xed_chip_features_t);
}

impl ChipFeatures {
    /// Create this chip features with those corresponding to the provided
    /// [`Chip`].
    pub fn from_chip(chip: Chip) -> Self {
        let mut features: xed_chip_features_t = unsafe { std::mem::zeroed() };
        unsafe { xed_get_chip_features(&mut features, chip.into_raw()) };
        Self(features)
    }

    /// Enable the features that correspond to the provided [`IsaSet`].
    pub fn enable_isa_set(&mut self, isa_set: IsaSet) {
        unsafe { xed_modify_chip_features(self.as_raw_mut(), isa_set.into_raw(), 1) }
    }

    /// Disable the features that correspond to the provided [`IsaSet`].
    pub fn disable_isa_set(&mut self, isa_set: IsaSet) {
        unsafe { xed_modify_chip_features(self.as_raw_mut(), isa_set.into_raw(), 0) }
    }
}
