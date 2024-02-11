use xed_sys::*;

crate::macros::xed_enum! {
    pub enum Syntax {
        /// XED disassembly syntax
        XED,

        /// AT&T SYS-V disassembly syntax
        ATT,

        /// Intel disassembly syntax
        INTEL,
    }
}
