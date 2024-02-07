//! Low-level bindings for XED.
//!
//! These are safe rust bindings for XED that map one-to-one with the XED C API.

mod address_width;
mod machine_mode;
mod state;

pub use self::address_width::AddressWidth;
pub use self::machine_mode::MachineMode;
pub use self::state::State;

/// Initialize the XED encode and decode tables.
///
/// This function must be called before using XED. It will be automatically
/// called when creating some of the required types in this library so you
/// should never need to call it directly.
pub fn init_tables() {
    use std::sync::OnceLock;

    static TABLES_INIT: OnceLock<()> = OnceLock::new();

    // SAFETY: The OnceLock ensures that we are not calling xed_tables_init
    //         concurrently.
    TABLES_INIT.get_or_init(|| unsafe { xed_sys::xed_tables_init() });
}
