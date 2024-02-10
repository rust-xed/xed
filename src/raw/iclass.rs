use std::ffi::c_uint;

use xed_sys::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct IClass(pub c_uint);

impl IClass {
    /// Return the maximum number of iforms for this iclass.
    ///
    /// > ## Note
    /// > This function will transparently initialize the global data tables if
    /// > they have not already been initialized.
    pub fn max_iform(self) -> u32 {
        crate::raw::init_tables();

        unsafe { xed_iform_max_per_iclass(self.0) }
    }
}

impl From<c_uint> for IClass {
    fn from(value: c_uint) -> Self {
        Self(value)
    }
}

impl From<IClass> for c_uint {
    fn from(value: IClass) -> Self {
        value.0
    }
}
