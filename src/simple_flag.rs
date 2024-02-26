use xed_sys::*;

use super::{
    raw::{AsRaw, FromRaw},
    FlagAction, FlagSet,
};

crate::macros::wrapper_type! {
    #[derive(FromRaw, AsRaw, AsRawMut, IntoRaw)]
    pub struct SimpleFlag(xed_simple_flag_t);
}

impl SimpleFlag {
    /// Get the specific flag actions.
    pub fn flag_actions(&self) -> &[FlagAction] {
        let len = unsafe { xed_simple_flag_get_nflags(self.as_raw()) as usize };
        let ptr = unsafe { xed_simple_flag_get_flag_action(self.as_raw(), 0) };

        // SAFETY: FlagAction is a #[repr(transparent)] wrapper around
        //         xed_flag_action_t.
        unsafe { std::slice::from_raw_parts(ptr as *const FlagAction, len) }
    }

    /// Indicates that the flags are only conditionally written.
    ///
    /// Usually MAY-writes of the flags are for instructions that are dependent
    /// on a REP count.
    pub fn may_write(&self) -> bool {
        unsafe { xed_simple_flag_get_may_write(self.as_raw()) != 0 }
    }

    /// The flags are always written.
    pub fn must_write(&self) -> bool {
        unsafe { xed_simple_flag_get_must_write(self.as_raw()) != 0 }
    }

    /// The union of bits for the read flags.
    pub fn read_flag_set(&self) -> &FlagSet {
        FlagSet::from_ref(unsafe { &*xed_simple_flag_get_read_flag_set(self.as_raw()) })
    }

    /// The union of bits for undefined flags.
    pub fn undefined_flag_set(&self) -> &FlagSet {
        FlagSet::from_ref(unsafe { &*xed_simple_flag_get_undefined_flag_set(self.as_raw()) })
    }

    /// The union of bits for written flags.
    pub fn written_flag_set(&self) -> &FlagSet {
        FlagSet::from_ref(unsafe { &*xed_simple_flag_get_written_flag_set(self.as_raw()) })
    }

    /// Whether the flags are read.
    pub fn reads_flags(&self) -> bool {
        unsafe { xed_simple_flag_reads_flags(self.as_raw()) != 0 }
    }

    /// Whether the flags are written.
    pub fn writes_flags(&self) -> bool {
        unsafe { xed_simple_flag_writes_flags(self.as_raw()) != 0 }
    }
}
