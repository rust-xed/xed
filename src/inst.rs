use xed_sys::*;

use super::{
    Attribute, Category, DecodedInst, Exception, Extension, IClass, IForm, IsaSet, NonTerminal,
    Operand, OperandAction, OperandElementXType, OperandType, OperandValue, OperandVisibility,
    OperandWidth, Register,
};

used_in_docs!(DecodedInst, OperandValue);

/// Constant information about a decoded instruction form.
#[repr(transparent)]
pub struct Inst(xed_inst_t);

impl Inst {
    pub fn from_ref(raw: &xed_inst_t) -> &Self {
        // SAFETY: Inst is #[repr(transparent)]
        unsafe { std::mem::transmute(raw) }
    }

    pub fn from_raw(raw: xed_inst_t) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> xed_inst_t {
        self.0
    }

    pub fn as_raw(&self) -> &xed_inst_t {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut xed_inst_t {
        &mut self.0
    }
}

impl Inst {
    /// The current privilege level (CPL) required for execution, 0 or 3.
    ///
    /// If the value is 0, then the instruction can only execute in ring 0.
    #[deprecated = "use DecodedInst::attribute(Attribute::RING0) instead"]
    pub fn cpl(&self) -> u32 {
        unsafe { xed_inst_cpl(self.as_raw()) }
    }

    pub fn operands(&self) -> &[InstOperand] {
        let len = unsafe { xed_inst_noperands(self.as_raw()) as usize };
        let ptr = unsafe { xed_inst_operand(self.as_raw(), 0) };

        unsafe { std::slice::from_raw_parts(ptr as *const InstOperand, len) }
    }

    pub fn iform(&self) -> IForm {
        unsafe { xed_inst_iform_enum(self.as_raw()) }.into()
    }

    pub fn iclass(&self) -> IClass {
        unsafe { xed_inst_iclass(self.as_raw()) }.into()
    }

    pub fn category(&self) -> Category {
        unsafe { xed_inst_category(self.as_raw()) }
            .try_into()
            .expect("instruction category was invalid")
    }

    pub fn extension(&self) -> Extension {
        unsafe { xed_inst_extension(self.as_raw()) }
            .try_into()
            .expect("instruction extension was invalid")
    }

    pub fn isa_set(&self) -> IsaSet {
        unsafe { xed_inst_isa_set(self.as_raw()) }
            .try_into()
            .expect("instruction isa_set was invalid")
    }

    pub fn flag_info_index(&self) -> u32 {
        unsafe { xed_inst_flag_info_index(self.as_raw()) }
    }

    pub fn attribute(&self, attr: Attribute) -> bool {
        unsafe { xed_inst_get_attribute(self.as_raw(), attr.into_raw()) != 0 }
    }

    pub fn attributes(&self) -> xed_attributes_t {
        unsafe { xed_inst_get_attributes(self.as_raw()) }
    }
}

impl Inst {
    /// Get the exceeption info if present for the specified instruction.
    ///
    /// This is currently only used for SSE and AVX instructions.
    pub fn exception(&self) -> Option<Exception> {
        unsafe { xed_inst_exception(self.as_raw()) }.try_into().ok()
    }
}

#[repr(transparent)]
pub struct InstOperand(xed_operand_t);

impl InstOperand {
    pub fn from_ref(raw: &xed_operand_t) -> &Self {
        // SAFETY: Inst is #[repr(transparent)]
        unsafe { std::mem::transmute(raw) }
    }

    pub fn from_raw(raw: xed_operand_t) -> Self {
        Self(raw)
    }

    pub fn into_raw(self) -> xed_operand_t {
        self.0
    }

    pub fn as_raw(&self) -> &xed_operand_t {
        &self.0
    }

    pub fn as_raw_mut(&mut self) -> &mut xed_operand_t {
        &mut self.0
    }
}

impl InstOperand {
    pub fn name(&self) -> Operand {
        unsafe { xed_operand_name(self.as_raw()) }
            .try_into()
            .expect("operand name was invalid")
    }

    pub fn visibility(&self) -> OperandVisibility {
        unsafe { xed_operand_operand_visibility(self.as_raw()) }
            .try_into()
            .expect("operand visibility was invalid")
    }

    /// The [`OperandType`] of the operand.
    ///
    /// This is probably not what you want.
    pub fn ty(&self) -> OperandType {
        unsafe { xed_operand_type(self.as_raw()) }
            .try_into()
            .expect("operand type was invalid")
    }

    /// The [`OperandElementXType`] of the operand.
    ///
    /// This is probably not what you want.
    pub fn xtype(&self) -> OperandElementXType {
        unsafe { xed_operand_xtype(self.as_raw()) }
            .try_into()
            .expect("operand xtype was invalid")
    }

    pub fn operand_width(&self) -> OperandWidth {
        unsafe { xed_operand_width(self.as_raw()) }
            .try_into()
            .expect("operand width was invalid")
    }

    /// The actual width of the operand in bits.
    ///
    /// # Parameters
    /// - `eosz` - The effective operand size of the instruction. Use 1/2/3 for
    ///   16/32/64 bits respectively. 0 is invalid.
    pub fn operand_width_bits(&self, eosz: u32) -> u32 {
        unsafe { xed_operand_width_bits(self.as_raw(), eosz) }
    }

    pub fn nonterminal_name(&self) -> NonTerminal {
        unsafe { xed_operand_nonterminal_name(self.as_raw()) }
            .try_into()
            .expect("operand nonterminal was invalid")
    }

    /// The implicit or suppressed register.
    ///
    /// Be careful with this one, it is probably not what you think it is. It is
    /// only used for hard-coded registers implicit in the instruction
    /// encoding. Most likely you want to get the [`Operand`] and then look
    /// up the instruction using [`DecodedInst::reg`]. The hard-coded
    /// registers are also available that way.
    pub fn reg(&self) -> Register {
        unsafe { xed_operand_reg(self.as_raw()) }
            .try_into()
            .expect("operand reg is invalid")
    }

    pub fn template_is_register(&self) -> bool {
        unsafe { xed_operand_template_is_register(self.as_raw()) != 0 }
    }

    /// These operands represent branch displacements, memory displacements, and
    /// various immediates.
    pub fn imm(&self) -> u32 {
        unsafe { xed_operand_imm(self.as_raw()) }
    }
}

impl InstOperand {
    /// Returns the raw R/W action.
    ///
    /// There are many cases for for conditional reads and writes. See
    /// [`OperandValue::action`].
    #[deprecated]
    pub fn rw(&self) -> OperandAction {
        unsafe { xed_operand_rw(self.as_raw()) }
            .try_into()
            .expect("operand action was invalid")
    }

    /// Whether the operand is read, including conditional reads.
    pub fn read(&self) -> bool {
        unsafe { xed_operand_read(self.as_raw()) != 0 }
    }

    /// Whether the operand is read-only, including conditional reads.
    pub fn read_only(&self) -> bool {
        unsafe { xed_operand_read_only(self.as_raw()) != 0 }
    }

    /// Whether the operand is written, including conditional writes.
    pub fn written(&self) -> bool {
        unsafe { xed_operand_written(self.as_raw()) != 0 }
    }

    /// Whether the operand is written-only, including conditional reads.
    pub fn written_only(&self) -> bool {
        unsafe { xed_operand_written_only(self.as_raw()) != 0 }
    }

    /// Whether the operand is read-and-written, including conditional reads and
    /// writes.
    pub fn read_and_written(&self) -> bool {
        unsafe { xed_operand_read_and_written(self.as_raw()) != 0 }
    }

    /// Whether the operand has a conditional read (may also write).
    pub fn conditional_read(&self) -> bool {
        unsafe { xed_operand_conditional_read(self.as_raw()) != 0 }
    }

    /// Whether the operand has a conditional write (may also read).
    pub fn conditional_write(&self) -> bool {
        unsafe { xed_operand_conditional_write(self.as_raw()) != 0 }
    }
}
