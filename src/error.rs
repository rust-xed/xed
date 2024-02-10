//! Various error types used within `xed`.

use core::fmt;

/// Error for when converting an integer to an enum fails.
#[derive(Copy, Clone, Debug)]
pub struct InvalidEnumValue {
    value: u32,
    name: &'static str,
}

impl InvalidEnumValue {
    pub(crate) fn new(value: u32, name: &'static str) -> Self {
        Self { value, name }
    }

    pub fn value(self) -> u32 {
        self.value
    }

    pub fn enum_name(&self) -> &str {
        self.name
    }
}

impl fmt::Display for InvalidEnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a valid value for {}", self.value, self.name)
    }
}
