use std::marker::PhantomData;

use xed_sys::*;

use crate::{
    Attribute, Category, Chip, Extension, IClass, IForm, IsaSet, Operand, OperandAction,
    OperandElementType, Register, SimpleFlag,
};

/// The main container for instructions.
///
/// It holds an array of operands with derived information from the decode and
/// also a valid [`Inst`] pointer which describes the operand templates and the
/// operand order.
///
/// [`Inst`]: super::Inst
#[repr(transparent)]
pub struct DecodedInst<'d> {
    inst: xed_decoded_inst_t,
    _marker: PhantomData<&'d [u8]>,
}

impl<'d> DecodedInst<'d> {
    /// Create a `DecodedInst` from the underlyingg [`xed_decoded_inst_t`].
    ///
    /// # Safety
    /// - The lifetime of the resulting `DecodedInst` must be less than the
    ///   lifetime of the instruction bytes that were used to decode into `raw`.
    pub unsafe fn from_raw(raw: xed_decoded_inst_t) -> Self {
        Self {
            inst: raw,
            _marker: PhantomData,
        }
    }

    pub fn as_raw(&self) -> &xed_decoded_inst_t {
        &self.inst
    }

    pub fn as_raw_mut(&mut self) -> &mut xed_decoded_inst_t {
        &mut self.inst
    }
}

// High-level accessors
impl<'d> DecodedInst<'d> {
    /// Get the category of this instruction.
    pub fn category(&self) -> Category {
        unsafe { xed_decoded_inst_get_category(self.as_raw()) }
            .try_into()
            .expect("stored category was not valid")
    }

    /// Get the extension that this instruction belongs to.
    pub fn extension(&self) -> Extension {
        unsafe { xed_decoded_inst_get_extension(self.as_raw()) }
            .try_into()
            .expect("extension value was invalid")
    }

    /// Get the ISA set that this instruction belongs to.
    pub fn isa_set(&self) -> IsaSet {
        unsafe { xed_decoded_inst_get_isa_set(self.as_raw()) }
            .try_into()
            .expect("isa_set value was invalid")
    }

    /// Get the instruction class of this instruction.
    pub fn iclass(&self) -> IClass {
        unsafe { xed_decoded_inst_get_iclass(self.as_raw()) }.into()
    }
}

// Attributes and properties
impl<'d> DecodedInst<'d> {
    /// Get whether the attrbute is defined for this instruction.
    pub fn attribute(&self, attr: Attribute) -> bool {
        unsafe { xed_decoded_inst_get_attribute(self.as_raw(), attr.into()) != 0 }
    }

    /// Get the whole attribute bitvector.
    pub fn attributes(&self) -> xed_attributes_t {
        unsafe { xed_decoded_inst_get_attributes(self.as_raw()) }
    }

    /// Whether this instruction is xacquire.
    pub fn is_xacquire(&self) -> bool {
        unsafe { xed_decoded_inst_is_xacquire(self.as_raw()) != 0 }
    }

    /// Whether this instruction is xrelease.
    pub fn is_xrelease(&self) -> bool {
        unsafe { xed_decoded_inst_is_xrelease(self.as_raw()) != 0 }
    }

    /// Whether this instruction ia an APX-promoted zero-upper (ZU) instruction.
    pub fn is_apx_zu(&self) -> bool {
        unsafe { xed_decoded_inst_is_apx_zu(self.as_raw()) != 0 }
    }

    /// Get the modrm byte.
    pub fn modrm(&self) -> u8 {
        unsafe { xed_decoded_inst_get_modrm(self.as_raw()) }
    }

    /// Whether the instruction uses destination masking.
    ///
    /// This returns false for blend operations that use their mask field as a
    /// control.
    pub fn masked_vector_operation(&self) -> bool {
        unsafe {
            // The source library wrongly has the argument here as a non-const pointer.
            // However, the argument is not modified.
            xed_decoded_inst_masked_vector_operation(self.as_raw() as *const _ as *mut _) != 0
        }
    }

    /// Returns 128, 256, or 512 for operations in the VEX, EVEX, or XOP
    /// encoding space and returns 0 for (most) nonvector operations.
    ///
    /// This is usually the content of the VEX.L or EVEX.LL field,
    /// reinterpreted. Some GPR instructions (like the BMI1/BMI2) are encoded in
    /// the VEX space and return non-zero values from this API.
    pub fn vector_length_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_vector_length_bits(self.as_raw()) }
    }

    /// The number of legacy prefixes included in this instruction.
    pub fn nprefixes(&self) -> u32 {
        unsafe { xed_decoded_inst_get_nprefixes(self.as_raw()) }
    }
}

/// Operands
impl<'d> DecodedInst<'d> {
    pub fn operands(&self) -> OperandValues {
        OperandValues::new(self)
    }
}

pub struct OperandValues<'a, 'd> {
    inst: &'a DecodedInst<'d>,
}

impl<'a, 'd> OperandValues<'a, 'd> {
    fn new(inst: &'a DecodedInst<'d>) -> Self {
        Self { inst }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { xed_decoded_inst_noperands(self.inst.as_raw()) as usize }
    }

    pub fn as_slice(&self) -> &'a [DecodedInst] {
        let ops = unsafe { xed_decoded_inst_operands_const(self.inst.as_raw()) };
        unsafe { std::slice::from_raw_parts(ops as *const DecodedInst, self.len()) }
    }

    pub fn get(&self, index: usize) -> Option<OperandValue<'a, 'd>> {
        if index >= self.len() {
            return None;
        }

        Some(OperandValue {
            inst: self.inst,
            index: index as u32,
        })
    }
}

pub struct OperandValue<'a, 'd> {
    inst: &'a DecodedInst<'d>,
    index: u32,
}

impl<'a> OperandValue<'a, '_> {
    /// The length of the operand.
    pub fn length(&self) -> u32 {
        unsafe { xed_decoded_inst_operand_length(self.inst.as_raw(), self.index) }
    }

    /// The length of the operand in bits.
    pub fn length_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_operand_length_bits(self.inst.as_raw(), self.index) }
    }

    /// The number of elements in the operand (for SSE and AVX operands).
    pub fn elements(&self) -> u32 {
        unsafe { xed_decoded_inst_operand_elements(self.inst.as_raw(), self.index) }
    }

    /// The size of an element in bits (for SSE and AVX operands).
    pub fn element_size_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_operand_element_size_bits(self.inst.as_raw(), self.index) }
    }

    /// The type of the elements in the operand (for SSE and AVX operands).
    pub fn element_type(&self) -> Option<OperandElementType> {
        unsafe { xed_decoded_inst_operand_element_type(self.inst.as_raw(), self.index) }
            .try_into()
            .ok()
    }

    /// Interpret the operand action in light of AVX512 masking and
    /// zeroing/merging.
    ///
    /// If masking and merging are used together, the dest operand may also be
    /// read and elements of the dest operation register may be conditionally
    /// written (so that input values live on the output register).
    pub fn action(&self) -> Option<OperandAction> {
        unsafe { xed_decoded_inst_operand_action(self.inst.as_raw(), self.index) }
            .try_into()
            .ok()
    }
}

// AVX512 masking
impl<'d> DecodedInst<'d> {
    /// Whether the instruction uses write masking.
    pub fn masking(&self) -> bool {
        unsafe { xed_decoded_inst_masking(self.as_raw()) != 0 }
    }

    /// Whether the instruction uses write-masking with merging.
    pub fn merging(&self) -> bool {
        unsafe { xed_decoded_inst_merging(self.as_raw()) != 0 }
    }

    /// Whether the instruction uses write-masking with zeroing.
    pub fn zeroing(&self) -> bool {
        unsafe { xed_decoded_inst_zeroing(self.as_raw()) != 0 }
    }

    /// The maximum number of elements processed.
    ///
    /// This only applies to AVX512 vector instructions.
    pub fn avx512_dest_elements(&self) -> u32 {
        unsafe { xed_decoded_inst_avx512_dest_elements(self.as_raw()) }
    }
}

impl<'d> DecodedInst<'d> {
    pub fn length(&self) -> usize {
        unsafe { xed_decoded_inst_get_length(self.as_raw()) as usize }
    }

    /// Get the bytes that make up the instruction.
    pub fn bytes(&self) -> &'d [u8] {
        let len = self.length();
        let ptr = unsafe { self.inst._byte_array._dec };

        unsafe { std::slice::from_raw_parts(ptr, len) }
    }
}

impl<'d> DecodedInst<'d> {
    pub fn machine_mode_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_machine_mode_bits(self.as_raw()) }
    }

    pub fn stack_address_mode_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_stack_address_mode_bits(self.as_raw()) }
    }

    /// The operand width in bits: 8/16/32/64.
    ///
    /// This is different than [`effective_operand_width`][0] which only returns
    /// 16/32/64. This factors in the BYTEOP attribute when computing its return
    /// value. Thios function provides informatino that is only useful for
    /// (scalable) GPR-operations. Individual operands have more spcific
    /// information available from [`OperandValue::element_size_bits`].
    ///
    /// [0]: OperandValue::effective_operand_width
    pub fn operand_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_operand_width(self.as_raw()) }
    }

    /// The user-specified chip name, or `None` if not set.
    pub fn input_chip(&self) -> Option<Chip> {
        unsafe { xed_decoded_inst_get_input_chip(self.as_raw()) }
            .try_into()
            .ok()
    }

    /// Whether this decoded instruction is valid for the specified [`Chip`].
    pub fn valid_for_chip(&self, chip: Chip) -> bool {
        unsafe { xed_decoded_inst_valid_for_chip(self.as_raw(), chip.into()) != 0 }
    }
}

// IFORM handling
impl<'d> DecodedInst<'d> {
    /// Get the instruction form value of this instruction.
    pub fn iform(&self) -> IForm {
        unsafe { xed_decoded_inst_get_iform_enum(self.as_raw()) }.into()
    }

    /// Get the instruction zero-based iform number based on masking the
    /// corresponding [`IForm`].
    ///
    /// This value is suitable for matching upon. The maximum value for a
    /// particular iclass is provided by [`IClass::max_iform`].
    pub fn iform_dispatch(&self) -> u32 {
        unsafe { xed_decoded_inst_get_iform_enum_dispatch(self.as_raw()) }
    }
}

pub struct MemoryOperands<'a, 'd>(OperandValues<'a, 'd>);

impl<'a, 'd> MemoryOperands<'a, 'd> {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { xed_decoded_inst_number_of_memory_operands(self.0.inst.as_raw()) as usize }
    }

    pub fn as_slice(&self) -> &'a [DecodedInst] {
        let ops = unsafe { xed_decoded_inst_operands_const(self.0.inst.as_raw()) };
        unsafe { std::slice::from_raw_parts(ops as *const DecodedInst, self.len()) }
    }

    pub fn get(&self, index: usize) -> Option<MemoryOperand<'a, 'd>> {
        if index >= self.len() {
            return None;
        }

        Some(MemoryOperand(OperandValue {
            inst: self.0.inst,
            index: index as u32,
        }))
    }
}

#[repr(transparent)]
pub struct MemoryOperand<'a, 'd>(OperandValue<'a, 'd>);

impl<'a, 'd> MemoryOperand<'a, 'd> {
    pub fn seg_reg(&self) -> Option<Register> {
        unsafe { xed_decoded_inst_get_seg_reg(self.0.inst.as_raw(), self.0.index) }
            .try_into()
            .ok()
    }

    pub fn base_reg(&self) -> Option<Register> {
        unsafe { xed_decoded_inst_get_base_reg(self.0.inst.as_raw(), self.0.index) }
            .try_into()
            .ok()
    }

    pub fn scale(&self) -> u32 {
        unsafe { xed_decoded_inst_get_scale(self.0.inst.as_raw(), self.0.index) }
    }

    pub fn displacement(&self) -> i64 {
        unsafe { xed_decoded_inst_get_memory_displacement(self.0.inst.as_raw(), self.0.index) }
    }

    pub fn displacement_width(&self) -> u32 {
        unsafe {
            xed_decoded_inst_get_memory_displacement_width(self.0.inst.as_raw(), self.0.index)
        }
    }

    pub fn displacement_width_bits(&self) -> u32 {
        unsafe {
            xed_decoded_inst_get_memory_displacement_width_bits(self.0.inst.as_raw(), self.0.index)
        }
    }

    pub fn mem_read(&self) -> bool {
        unsafe { xed_decoded_inst_mem_read(self.0.inst.as_raw(), self.0.index) != 0 }
    }

    pub fn mem_written(&self) -> bool {
        unsafe { xed_decoded_inst_mem_written(self.0.inst.as_raw(), self.0.index) != 0 }
    }

    pub fn mem_written_only(&self) -> bool {
        unsafe { xed_decoded_inst_mem_written_only(self.0.inst.as_raw(), self.0.index) != 0 }
    }

    pub fn operand_length(&self) -> u32 {
        unsafe { xed_decoded_inst_get_memory_operand_length(self.0.inst.as_raw(), self.0.index) }
    }

    /// The addressing width in bits 16/32/64.
    ///
    /// This factors in things like whether or not the reference is an implicit
    /// stack push/pop, the machine mode, and 67 prefixes if present.
    pub fn address_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_memop_address_width(self.0.inst.as_raw(), self.0.index) }
    }

    pub fn index_reg(&self) -> Option<Register> {
        unsafe { xed_decoded_inst_get_index_reg(self.0.inst.as_raw(), self.0.index) }
            .try_into()
            .ok()
    }
}

impl<'d> DecodedInst<'d> {
    pub fn memory_operands(&self) -> MemoryOperands {
        MemoryOperands(self.operands())
    }

    pub fn branch_displacement(&self) -> i64 {
        unsafe { xed_decoded_inst_get_branch_displacement(self.as_raw()) }
    }

    pub fn branch_displacement_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_branch_displacement_width(self.as_raw()) }
    }

    pub fn branch_displacement_width_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_branch_displacement_width_bits(self.as_raw()) }
    }

    pub fn unsigned_immediate(&self) -> u64 {
        unsafe { xed_decoded_inst_get_unsigned_immediate(self.as_raw()) }
    }

    /// Whether the first immediate (IMM0) is signed.
    pub fn immediate_is_signed(&self) -> bool {
        unsafe { xed_decoded_inst_get_immediate_is_signed(self.as_raw()) != 0 }
    }

    /// The width of the first immediate in bytes.
    pub fn immediate_width(&self) -> u32 {
        unsafe { xed_decoded_inst_get_immediate_width(self.as_raw()) }
    }

    /// The width of the first immediate in bits.
    pub fn immediate_width_bits(&self) -> u32 {
        unsafe { xed_decoded_inst_get_immediate_width_bits(self.as_raw()) }
    }

    pub fn signed_immediate(&self) -> i32 {
        unsafe { xed_decoded_inst_get_signed_immediate(self.as_raw()) }
    }

    pub fn second_immediate(&self) -> u8 {
        unsafe { xed_decoded_inst_get_second_immediate(self.as_raw()) }
    }

    pub fn reg(&self, operand: Operand) -> Option<Register> {
        unsafe { xed_decoded_inst_get_reg(self.as_raw(), operand.into_raw()) }
            .try_into()
            .ok()
    }

    /// Get the DFV register if one of the instruction's operands is a "default
    /// flags values" pseudo-register.
    pub fn dfv_reg(&self) -> Option<Register> {
        unsafe { xed_decoded_inst_get_dfv_reg(self.as_raw()) }
            .try_into()
            .ok()
    }

    /// Get information about the flags, if the instruction uses them.
    ///
    /// For some shifts/rotates, XED puts a flags operands in the operand array
    /// before it knows if the flags are used because of mode-dependent masking
    /// effects on the immediate. This can mean that this method can return
    /// `Some` even if [`uses_rflags`] returns false.
    ///
    /// [`uses_rflags`]: Self::uses_rflags
    pub fn rflags_info(&self) -> Option<&SimpleFlag> {
        unsafe {
            xed_decoded_inst_get_rflags_info(self.as_raw())
                .as_ref()
                .map(SimpleFlag::from_ref)
        }
    }

    /// Whether the flags are read or written.
    ///
    /// For some shifts/rotates, XED puts a flags operands in the operand array
    /// before it knows if the flags are used because of mode-dependent masking
    /// effects on the immediate. This can mean that [`rflags_info`] can return
    /// `Some` even if this method returns false.
    ///
    /// [`rflags_info`]: Self::rflags_info
    pub fn uses_rflags(&self) -> bool {
        unsafe { xed_decoded_inst_uses_rflags(self.as_raw()) != 0 }
    }

    pub fn conditionally_writes_registers(&self) -> bool {
        unsafe { xed_decoded_inst_conditionally_writes_registers(self.as_raw()) != 0 }
    }

    pub fn is_prefetch(&self) -> bool {
        unsafe { xed_decoded_inst_is_prefetch(self.as_raw()) != 0 }
    }

    pub fn is_broadcast(&self) -> bool {
        unsafe { xed_decoded_inst_is_broadcast(self.as_raw()) != 0 }
    }

    pub fn is_broadcast_instruction(&self) -> bool {
        unsafe { xed_decoded_inst_is_broadcast_instruction(self.as_raw()) != 0 }
    }

    pub fn uses_embedded_braocast(&self) -> bool {
        unsafe { xed_decoded_inst_uses_embedded_broadcast(self.as_raw()) != 0 }
    }
}

// Classifiers
impl<'d> DecodedInst<'d> {
    /// True for APX instructions.
    ///
    /// Includes instructions with EGPRs, REX2, and encodings that eare treated
    /// as illegal on non-APX systems.
    pub fn classify_apx(&self) -> bool {
        unsafe { xed_classify_apx(self.as_raw()) != 0 }
    }

    /// True for AMX instructions.
    pub fn classify_amx(&self) -> bool {
        unsafe { xed_classify_amx(self.as_raw()) != 0 }
    }

    /// True for AVX/AVX2 SIMD VEX-encoded operations.
    ///
    /// Does not include BMI/BMI2 instructions.
    pub fn classify_avx(&self) -> bool {
        unsafe { xed_classify_avx(self.as_raw()) != 0 }
    }

    /// True for AVX512 (EVEX-encoded) SIMD and (VEX encoded) K-mask
    /// instructions.
    pub fn classify_avx512(&self) -> bool {
        unsafe { xed_classify_avx512(self.as_raw()) != 0 }
    }

    /// True for AVX512 (VEX-encoded) K-mask operations.
    pub fn classify_avx512_maskop(&self) -> bool {
        unsafe { xed_classify_avx512_maskop(self.as_raw()) != 0 }
    }

    /// True for SSE/SSE2/etc. SIMD operations. Includes AES and PCLMULQDQ.
    pub fn classify_sse(&self) -> bool {
        unsafe { xed_classify_sse(self.as_raw()) != 0 }
    }
}
