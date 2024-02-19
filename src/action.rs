use xed_sys::*;

use super::{
    raw::{AsRaw, IntoRaw},
    Flag,
};

crate::macros::xed_enum! {
    pub enum Action => XED_FLAG_ACTION {
        /// Undefined (treated as a write).
        U => XED_FLAG_ACTION_u,

        /// Test (read)
        TST => XED_FLAG_ACTION_tst,

        /// Modification (write)
        MOD => XED_FLAG_ACTION_mod,

        /// Value will be zero (write)
        ZERO => XED_FLAG_ACTION_0,

        /// Value comes from the stack (write)
        POP => XED_FLAG_ACTION_pop,

        /// Value comes from AH (write)
        AH => XED_FLAG_ACTION_ah,

        /// Value will be 1 (write)
        ONE => XED_FLAG_ACTION_1,
    }
}

impl Action {
    /// Test to see if this action is a read.
    pub fn is_read(self) -> bool {
        unsafe { xed_flag_action_read_action(self.into_raw()) != 0 }
    }

    /// Test to see if this action is a write.
    pub fn is_write(self) -> bool {
        unsafe { xed_flag_action_write_action(self.into_raw()) != 0 }
    }
}

crate::macros::wrapper_type! {
    #[derive(FromRaw, AsRaw, AsRawMut, IntoRaw)]
    pub struct FlagAction(xed_flag_action_t);
}

impl FlagAction {
    /// The action performed by this flag.
    pub fn action(&self) -> Action {
        // Note: xed takes an index i but it is entirely unused by the actual function
        //       with no documentation as to what it is supposed to refer to.
        unsafe { xed_flag_action_get_action(self.as_raw(), 0) }
            .try_into()
            .expect("action value was invalid")
    }

    /// The name of the flag.
    pub fn flag_name(&self) -> Flag {
        unsafe { xed_flag_action_get_flag_name(self.as_raw()) }
            .try_into()
            .expect("flag value was invalid")
    }
}
