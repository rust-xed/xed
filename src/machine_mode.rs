use xed_sys::*;

use crate::macros::wrapper_enum;

wrapper_enum! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum MachineMode {
        /// 64b operating mode.
        Long64 = XED_MACHINE_MODE_LONG_64,

        /// 32b protected mode.
        LongCompat32 = XED_MACHINE_MODE_LONG_COMPAT_32,

        /// 16b protected mode.
        LongCompat16 = XED_MACHINE_MODE_LONG_COMPAT_16,

        /// 32b protected mode.
        Legacy32 = XED_MACHINE_MODE_LEGACY_32,

        /// 16b protected mode.
        Legacy16 = XED_MACHINE_MODE_LEGACY_16,

        /// 16b real mode.
        Real16 = XED_MACHINE_MODE_REAL_16,

        /// 32b real mode.
        Real32 = XED_MACHINE_MODE_REAL_32,
    }
}
