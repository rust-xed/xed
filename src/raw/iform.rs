use std::ffi::{c_uint, CStr};

use xed_sys::*;

use super::{Category, Extension, IsaSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct IForm(pub c_uint);

impl IForm {
    /// Get the instruction category for this iform.
    ///
    /// > ## Note
    /// > This function will transparently initialize the global data tables if
    /// > they have not already been initialized.
    pub fn category(&self) -> Category {
        super::init_tables();

        unsafe { xed_iform_to_category(self.0) }
            .try_into()
            .expect("category value was invalid")
    }

    /// Get the ISA extension for this enum.
    ///
    /// > ## Note
    /// > This function will transparently initialize the global data tables if
    /// > they have not already been initialized.
    pub fn extension(&self) -> Extension {
        super::init_tables();

        unsafe { xed_iform_to_extension(self.0) }
            .try_into()
            .expect("extension value was invalid")
    }

    /// Get the name of this instruction in AT&T SYSV syntax.
    ///
    /// These names are slightly ambiguous but are usually more useful to people
    /// using them.
    pub fn name_att(&self) -> &'static str {
        let name = unsafe { CStr::from_ptr(xed_iform_to_iclass_string_att(self.0)) };

        match name.to_str() {
            Ok(name) => name,
            Err(_) => unreachable!("iform name {name:?} contained invalid utf-8"),
        }
    }

    /// Get the name of this instruction in Intel syntax.
    ///
    /// These names are slightly ambiguous but are usually more useful to people
    /// using them.
    pub fn name_intel(&self) -> &'static str {
        let name = unsafe { CStr::from_ptr(xed_iform_to_iclass_string_intel(self.0)) };

        match name.to_str() {
            Ok(name) => name,
            Err(_) => unreachable!("iform name {name:?} contained invalid utf-8"),
        }
    }

    /// Get the ISA set for this iform.
    ///
    /// > ## Note
    /// > This function will transparently initialize the global data tables if
    /// > they have not already been initialized.
    pub fn isa_set(&self) -> IsaSet {
        super::init_tables();

        unsafe { xed_iform_to_isa_set(self.0) }
            .try_into()
            .expect("isa_set value was invalid")
    }
}

impl From<c_uint> for IForm {
    fn from(value: c_uint) -> Self {
        Self(value)
    }
}

impl From<IForm> for c_uint {
    fn from(value: IForm) -> Self {
        value.0
    }
}
