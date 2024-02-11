use xed_sys::*;

crate::macros::xed_enum! {
    pub enum OperandElementType => XED_OPERAND_ELEMENT_TYPE {
        /// Unsigned integer
        UINT,

        /// Signed integer
        INT,

        /// 32b FP single precision
        SINGLE,

        /// 64b FP double precision
        DOUBLE,

        /// 80b FP x87
        LONGDOUBLE,

        /// 80b decimal BCD
        LONGBCD,

        /// A structure of various fields.
        STRUCT,

        /// Depends on other fields in the instruction.
        VARIABLE,

        /// 16b floating point
        FLOAT16,

        /// bfloat16 floating point
        BFLOAT16,

        /// 8-bit integer
        INT8,

        /// 8-bit unsigned integer
        UINT8,
    }
}
