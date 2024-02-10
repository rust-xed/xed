use std::ffi::CStr;
use std::fmt;

pub(crate) struct DisplayCStr<'a>(pub &'a CStr);

impl fmt::Display for DisplayCStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_string_lossy())
    }
}
