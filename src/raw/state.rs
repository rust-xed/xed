use std::fmt;

use xed_sys::*;

use super::{AddressWidth, MachineMode};

#[derive(Copy, Clone)]
pub struct State(xed_state_t);

impl State {
    /// Create a new state from a machine mode and stack address width.
    ///
    /// # Panics
    /// - If `mmode` is [`MachineMode::Long64`] and `stack_addr_width` is not
    ///   [`AddressWidth::W64b`].
    pub fn new(mmode: MachineMode, stack_addr_width: AddressWidth) -> Self {
        if mmode == MachineMode::Long64 {
            assert_eq!(stack_addr_width, AddressWidth::W64b);
        }

        let mut state: xed_state_t = unsafe { std::mem::zeroed() };
        unsafe { xed_state_init2(&mut state, mmode.into(), stack_addr_width.into()) };

        Self(state)
    }

    #[inline]
    pub fn as_raw(&self) -> &xed_state_t {
        &self.0
    }

    #[inline]
    pub fn as_raw_mut(&mut self) -> &mut xed_state_t {
        &mut self.0
    }

    /// Get the address width.
    pub fn address_width(&self) -> AddressWidth {
        unsafe { xed_state_get_address_width(self.as_raw()) }
            .try_into()
            .expect("address width was not a valid address width")
    }

    /// Get the machine mode.
    pub fn machine_mode(&self) -> MachineMode {
        self.0
            .mmode
            .try_into()
            .expect("machine mode was not a valid machine mode")
    }

    /// Get the stack address width.
    pub fn stack_address_width(&self) -> AddressWidth {
        self.0
            .stack_addr_width
            .try_into()
            .expect("stack address width was not a valid address width")
    }

    /// Set the machine mode.
    ///
    /// The machine mode indicates the default data operand size.
    pub fn set_machine_mode(&mut self, mode: MachineMode) {
        self.0.mmode = mode.into();
    }

    /// Set the stack address width.
    pub fn set_stack_address_width(&mut self, addr_width: AddressWidth) {
        self.0.stack_addr_width = addr_width.into();
    }

    /// Is this state's machine mode [`MachineMode::Long64`]?
    pub fn is_long64_mode(&self) -> bool {
        unsafe { xed_state_long64_mode(self.as_raw()) != 0 }
    }

    /// Is this state in real mode?
    pub fn is_real_mode(&self) -> bool {
        unsafe { xed_state_real_mode(self.as_raw()) != 0 }
    }

    /// Is this state's address width 16b?
    pub fn is_mode_width_16(&self) -> bool {
        unsafe { xed_state_mode_width_16(self.as_raw()) != 0 }
    }

    /// Is this state's address width 32b?
    pub fn is_mode_width_32(&self) -> bool {
        unsafe { xed_state_mode_width_32(self.as_raw()) != 0 }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg = f.debug_struct("State");

        match MachineMode::try_from(self.0.mmode) {
            Ok(mmode) => dbg.field("machine_mode", &mmode),
            Err(e) => dbg.field("machine_mode", &e.value()),
        };

        match AddressWidth::try_from(self.0.stack_addr_width) {
            Ok(width) => dbg.field("stack_address_width", &width),
            Err(e) => dbg.field("stack_address_width", &e.value()),
        };

        dbg.finish()
    }
}
