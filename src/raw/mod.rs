//! Low-level bindings for XED.
//!
//! These are safe rust bindings for XED that map one-to-one with the XED C API.

mod action;
mod address_width;
mod attribute;
mod category;
mod chip;
mod decoded_inst;
mod extension;
mod flag;
mod flag_set;
mod iclass;
mod iform;
mod isa_set;
mod machine_mode;
mod operand;
mod operand_action;
mod operand_element_type;
mod register;
mod simple_flag;
mod state;

pub use self::action::{Action, FlagAction};
pub use self::address_width::AddressWidth;
pub use self::attribute::Attribute;
pub use self::category::Category;
pub use self::chip::Chip;
pub use self::decoded_inst::{
    DecodedInst, MemoryOperand, MemoryOperands, OperandValue, OperandValues,
};
pub use self::extension::Extension;
pub use self::flag::Flag;
pub use self::flag_set::FlagSet;
pub use self::iclass::IClass;
pub use self::iform::IForm;
pub use self::isa_set::IsaSet;
pub use self::machine_mode::MachineMode;
pub use self::operand::Operand;
pub use self::operand_action::OperandAction;
pub use self::operand_element_type::OperandElementType;
pub use self::register::Register;
pub use self::simple_flag::SimpleFlag;
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
