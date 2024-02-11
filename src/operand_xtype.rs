use xed_sys::*;

crate::macros::xed_enum! {
    pub enum OperandElementXType => XED_OPERAND_XTYPE {
        _2BF16 => XED_OPERAND_XTYPE_2BF16,
        _2F16 => XED_OPERAND_XTYPE_2F16,
        _2I16 => XED_OPERAND_XTYPE_2I16,
        _2U16 => XED_OPERAND_XTYPE_2U16,
        _4I8 => XED_OPERAND_XTYPE_4I8,
        _4U8 => XED_OPERAND_XTYPE_4U8,
        B80,
        F16,
        F32,
        F64,
        F80,
        I1,
        I128,
        I16,
        I32,
        I64,
        I8,
        INT,
        STRUCT,
        U128,
        U16,
        U256,
        U32,
        U64,
        U8,
        UINT,
        VAR
    }

    name = xed_operand_element_xtype_enum_t;
}
