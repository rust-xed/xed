//! Various error types used within `xed`.

use core::fmt;

/// Error for when converting an integer to an enum fails.
#[derive(Copy, Clone, Debug)]
pub struct InvalidEnumValue<T> {
    value: T,
    name: &'static str,
}

impl<T> InvalidEnumValue<T> {
    pub(crate) fn new(value: T, name: &'static str) -> Self {
        Self { value, name }
    }

    pub fn value(self) -> T {
        self.value
    }

    pub fn enum_name(&self) -> &str {
        self.name
    }
}

impl<T: fmt::Debug> fmt::Display for InvalidEnumValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a valid value for {}", self.value, self.name)
    }
}
