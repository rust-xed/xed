use std::fmt;

use xed_sys::*;

crate::macros::xed_enum! {
    /// Errors emitted by various XED functions.
    pub enum Error {
        //// There were not enough bytes in t he given buffer.
        BUFFER_TOO_SHORT,

        /// XED could not decode the given instruction.
        GENERAL_ERROR,

        /// The instruction is not valid for the specified chip.
        INVALID_FOR_CHIP,

        /// XED could not decode the given instruction because an invalid
        /// register encodign was used.
        BAD_REGISTER,

        /// A lock prefix was found where none was allowed.
        BAD_LOCK_PREFIX,

        /// An F2 or F3 prefix was found where none is allowed.
        BAD_REP_PREFIX,

        /// A 66, F2, or F3 prefix was found where none is allowed.
        BAD_LEGACY_PREFIX,

        /// A REX prefix was found where none is allowed
        BAD_REX_PREFIX,

        /// An illegal value for the MAP field was detected in the instruction.
        BAD_MAP,

        /// EVEX.V'=0 was detected in a non-64b mode instruction.
        BAD_EVEX_V_PRIME,

        /// EVEX.Z != 0 when EVEX.aaa == 0
        BAD_EVEX_Z_NO_MASKING,

        /// The output pointer for xed_agen was zero.
        NO_OUTPUT_POINTER,

        /// One or both of the callbacks for xed_agen were missing.
        NO_AGEN_CALL_BACK_REGISTERED,

        /// Memop indices must be 0 or 1.
        BAD_MEMOP_INDEX,

        /// The register or segment callback for xed_agen experienced a problem.
        CALLBACK_PROBLEM,

        /// The index, dest, and mask regs for AVX2 gathers must be different.
        GATHER_REGS,

        /// Full decode of the instruction would exceed 15B.
        INSTR_TOO_LONG,

        /// The instruction was not valid for the specified mode.
        INVALID_MODE,

        /// EVEX.LL must not equal 3 unless using embedded rounding.
        BAD_EVEX_LL,

        /// Some registers must not match for this instruction.
        ///
        /// Example: source with dest or dest with dest.
        BAD_REG_MATCH,
    }

    invalid = XED_ERROR_NONE;
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Maybe xed has some error strings for these?
        <Self as fmt::Debug>::fmt(self, f)
    }
}

impl std::error::Error for Error {}

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

impl std::error::Error for InvalidEnumValue {}

// #[derive(Copy, Clone, Debug)]
// pub struct DisassembleError(());

// impl fmt::Display for DisassembleError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str("")
//     }
// }
