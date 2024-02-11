use xed_sys::*;

crate::macros::xed_enum! {
    pub enum OperandType {
        ERROR,
        IMM,
        IMM_CONST,
        NT_LOOKUP_FN,
        NT_LOOKUP_FN2,
        NT_LOOKUP_FN4,
        REG,
    }
}
