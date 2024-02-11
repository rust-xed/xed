use xed_sys::*;

crate::macros::xed_enum! {
    pub enum OperandVisibility => XED_OPVIS {
        EXPLICIT,
        IMPLICIT,
        SUPPRESSED,
    }

    name = xed_operand_visibility_enum_t;
}
