use xed_sys::*;

crate::macros::xed_enum! {
    pub enum OperandAction {
        /// Read and written (must write).
        RW,

        /// Read-only.
        R,

        /// Write-only.
        W,

        /// Read and conditionally written (may write).
        RCW,

        /// Conditionally written.
        CW,

        /// Conditionally read, always written (must write)
        CRW,

        /// Conditional read.
        CR,

    }
}
