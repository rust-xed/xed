use xed_sys::*;

use super::{Chip, IsaSet};

#[repr(transparent)]
pub struct ChipFeatures(xed_chip_features_t);

impl ChipFeatures {
    pub fn from_ref(raw: &xed_chip_features_t) -> &Self {
        // SAFETY: ChipFeatures is #[repr(transparent)]
        unsafe { std::mem::transmute(raw) }
    }

    pub fn from_raw(raw: xed_chip_features_t) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> xed_chip_features_t {
        self.0
    }

    pub fn as_raw(&self) -> &xed_chip_features_t {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut xed_chip_features_t {
        &mut self.0
    }
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
