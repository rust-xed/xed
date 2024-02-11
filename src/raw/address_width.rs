use xed_sys::*;

use crate::macros::wrapper_enum;

wrapper_enum! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum AddressWidth {
        /// 16b addressing
        Word = XED_ADDRESS_WIDTH_16b,

        /// 32b addressing
        DWord = XED_ADDRESS_WIDTH_32b,

        /// 64b addressing
        QWord = XED_ADDRESS_WIDTH_64b,
    }
}
