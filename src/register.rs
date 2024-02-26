use xed_sys::*;

use crate::raw::IntoRaw;

crate::macros::xed_enum! {
    pub enum Register => XED_REG {
        BNDCFGU,
        BNDSTATUS,
        BND0,
        BND1,
        BND2,
        BND3,
        CR0,
        CR1,
        CR2,
        CR3,
        CR4,
        CR5,
        CR6,
        CR7,
        CR8,
        CR9,
        CR10,
        CR11,
        CR12,
        CR13,
        CR14,
        CR15,
        DR0,
        DR1,
        DR2,
        DR3,
        DR4,
        DR5,
        DR6,
        DR7,
        FLAGS,
        EFLAGS,
        RFLAGS,
        AX,
        CX,
        DX,
        BX,
        SP,
        BP,
        SI,
        DI,
        R8W,
        R9W,
        R10W,
        R11W,
        R12W,
        R13W,
        R14W,
        R15W,
        R16W,
        R17W,
        R18W,
        R19W,
        R20W,
        R21W,
        R22W,
        R23W,
        R24W,
        R25W,
        R26W,
        R27W,
        R28W,
        R29W,
        R30W,
        R31W,
        EAX,
        ECX,
        EDX,
        EBX,
        ESP,
        EBP,
        ESI,
        EDI,
        R8D,
        R9D,
        R10D,
        R11D,
        R12D,
        R13D,
        R14D,
        R15D,
        R16D,
        R17D,
        R18D,
        R19D,
        R20D,
        R21D,
        R22D,
        R23D,
        R24D,
        R25D,
        R26D,
        R27D,
        R28D,
        R29D,
        R30D,
        R31D,
        RAX,
        RCX,
        RDX,
        RBX,
        RSP,
        RBP,
        RSI,
        RDI,
        R8,
        R9,
        R10,
        R11,
        R12,
        R13,
        R14,
        R15,
        R16,
        R17,
        R18,
        R19,
        R20,
        R21,
        R22,
        R23,
        R24,
        R25,
        R26,
        R27,
        R28,
        R29,
        R30,
        R31,
        AL,
        CL,
        DL,
        BL,
        SPL,
        BPL,
        SIL,
        DIL,
        R8B,
        R9B,
        R10B,
        R11B,
        R12B,
        R13B,
        R14B,
        R15B,
        R16B,
        R17B,
        R18B,
        R19B,
        R20B,
        R21B,
        R22B,
        R23B,
        R24B,
        R25B,
        R26B,
        R27B,
        R28B,
        R29B,
        R30B,
        R31B,
        AH,
        CH,
        DH,
        BH,
        ERROR,
        RIP,
        EIP,
        IP,
        K0,
        K1,
        K2,
        K3,
        K4,
        K5,
        K6,
        K7,
        MMX0,
        MMX1,
        MMX2,
        MMX3,
        MMX4,
        MMX5,
        MMX6,
        MMX7,
        SSP,
        IA32_U_CET,
        MXCSR,
        STACKPUSH,
        STACKPOP,
        GDTR,
        LDTR,
        IDTR,
        TR,
        TSC,
        TSCAUX,
        MSRS,
        FSBASE,
        GSBASE,
        TILECONFIG,
        IA32_KERNEL_GS_BASE,
        DFV0,
        DFV1,
        DFV2,
        DFV3,
        DFV4,
        DFV5,
        DFV6,
        DFV7,
        DFV8,
        DFV9,
        DFV10,
        DFV11,
        DFV12,
        DFV13,
        DFV14,
        DFV15,
        X87CONTROL,
        X87STATUS,
        X87TAG,
        X87PUSH,
        X87POP,
        X87POP2,
        X87OPCODE,
        X87LASTCS,
        X87LASTIP,
        X87LASTDS,
        X87LASTDP,
        ES,
        CS,
        SS,
        DS,
        FS,
        GS,
        TMP0,
        TMP1,
        TMP2,
        TMP3,
        TMP4,
        TMP5,
        TMP6,
        TMP7,
        TMP8,
        TMP9,
        TMP10,
        TMP11,
        TMP12,
        TMP13,
        TMP14,
        TMP15,
        TMM0,
        TMM1,
        TMM2,
        TMM3,
        TMM4,
        TMM5,
        TMM6,
        TMM7,
        UIF,
        ST0,
        ST1,
        ST2,
        ST3,
        ST4,
        ST5,
        ST6,
        ST7,
        XCR0,
        XMM0,
        XMM1,
        XMM2,
        XMM3,
        XMM4,
        XMM5,
        XMM6,
        XMM7,
        XMM8,
        XMM9,
        XMM10,
        XMM11,
        XMM12,
        XMM13,
        XMM14,
        XMM15,
        XMM16,
        XMM17,
        XMM18,
        XMM19,
        XMM20,
        XMM21,
        XMM22,
        XMM23,
        XMM24,
        XMM25,
        XMM26,
        XMM27,
        XMM28,
        XMM29,
        XMM30,
        XMM31,
        YMM0,
        YMM1,
        YMM2,
        YMM3,
        YMM4,
        YMM5,
        YMM6,
        YMM7,
        YMM8,
        YMM9,
        YMM10,
        YMM11,
        YMM12,
        YMM13,
        YMM14,
        YMM15,
        YMM16,
        YMM17,
        YMM18,
        YMM19,
        YMM20,
        YMM21,
        YMM22,
        YMM23,
        YMM24,
        YMM25,
        YMM26,
        YMM27,
        YMM28,
        YMM29,
        YMM30,
        YMM31,
        ZMM0,
        ZMM1,
        ZMM2,
        ZMM3,
        ZMM4,
        ZMM5,
        ZMM6,
        ZMM7,
        ZMM8,
        ZMM9,
        ZMM10,
        ZMM11,
        ZMM12,
        ZMM13,
        ZMM14,
        ZMM15,
        ZMM16,
        ZMM17,
        ZMM18,
        ZMM19,
        ZMM20,
        ZMM21,
        ZMM22,
        ZMM23,
        ZMM24,
        ZMM25,
        ZMM26,
        ZMM27,
        ZMM28,
        ZMM29,
        ZMM30,
        ZMM31,
    }
}

impl Register {
    pub fn is_bndcfg(self) -> bool {
        (XED_REG_BNDCFG_FIRST..=XED_REG_BNDCFG_LAST).contains(&self.into_raw())
    }

    pub fn is_bndstat(self) -> bool {
        (XED_REG_BNDSTAT_FIRST..=XED_REG_BNDSTAT_LAST).contains(&self.into_raw())
    }

    pub fn is_bound(self) -> bool {
        (XED_REG_BOUND_FIRST..=XED_REG_BOUND_LAST).contains(&self.into_raw())
    }

    pub fn is_cr(self) -> bool {
        (XED_REG_CR_FIRST..=XED_REG_CR_LAST).contains(&self.into_raw())
    }

    pub fn is_dr(self) -> bool {
        (XED_REG_DR_FIRST..=XED_REG_DR_LAST).contains(&self.into_raw())
    }

    pub fn is_flags(self) -> bool {
        (XED_REG_FLAGS_FIRST..=XED_REG_FLAGS_LAST).contains(&self.into_raw())
    }

    pub fn is_gpr16(self) -> bool {
        (XED_REG_GPR16_FIRST..=XED_REG_GPR16_LAST).contains(&self.into_raw())
    }

    pub fn is_gpr32(self) -> bool {
        (XED_REG_GPR32_FIRST..=XED_REG_GPR32_LAST).contains(&self.into_raw())
    }

    pub fn is_gpr8(self) -> bool {
        (XED_REG_GPR8_FIRST..=XED_REG_GPR8_LAST).contains(&self.into_raw())
    }

    pub fn is_gpr8h(self) -> bool {
        (XED_REG_GPR8h_FIRST..=XED_REG_GPR8h_LAST).contains(&self.into_raw())
    }

    pub fn is_ip(self) -> bool {
        (XED_REG_IP_FIRST..=XED_REG_IP_LAST).contains(&self.into_raw())
    }

    pub fn is_mask(self) -> bool {
        (XED_REG_MASK_FIRST..=XED_REG_MASK_LAST).contains(&self.into_raw())
    }

    pub fn is_mmx(self) -> bool {
        (XED_REG_MMX_FIRST..=XED_REG_MMX_LAST).contains(&self.into_raw())
    }

    pub fn is_msr(self) -> bool {
        (XED_REG_MSR_FIRST..=XED_REG_MSR_LAST).contains(&self.into_raw())
    }

    pub fn is_mxcsr(self) -> bool {
        (XED_REG_MXCSR_FIRST..=XED_REG_MXCSR_LAST).contains(&self.into_raw())
    }

    pub fn is_pseudo(self) -> bool {
        (XED_REG_PSEUDO_FIRST..=XED_REG_PSEUDO_LAST).contains(&self.into_raw())
    }

    pub fn is_pseudo_x87(self) -> bool {
        (XED_REG_PSEUDOX87_FIRST..=XED_REG_PSEUDOX87_LAST).contains(&self.into_raw())
    }

    pub fn is_sr(self) -> bool {
        (XED_REG_SR_FIRST..=XED_REG_SR_LAST).contains(&self.into_raw())
    }

    pub fn is_tmp(self) -> bool {
        (XED_REG_TMP_FIRST..=XED_REG_TMP_LAST).contains(&self.into_raw())
    }

    pub fn is_treg(self) -> bool {
        (XED_REG_TREG_FIRST..=XED_REG_TREG_LAST).contains(&self.into_raw())
    }

    pub fn is_uif(self) -> bool {
        (XED_REG_UIF_FIRST..=XED_REG_UIF_LAST).contains(&self.into_raw())
    }

    pub fn is_x87(self) -> bool {
        (XED_REG_X87_FIRST..=XED_REG_X87_LAST).contains(&self.into_raw())
    }

    pub fn is_xcr(self) -> bool {
        (XED_REG_XCR_FIRST..=XED_REG_XCR_LAST).contains(&self.into_raw())
    }

    pub fn is_xmm(self) -> bool {
        (XED_REG_XMM_FIRST..=XED_REG_XMM_LAST).contains(&self.into_raw())
    }

    pub fn is_ymm(self) -> bool {
        (XED_REG_YMM_FIRST..=XED_REG_YMM_LAST).contains(&self.into_raw())
    }

    pub fn is_zmm(self) -> bool {
        (XED_REG_ZMM_FIRST..=XED_REG_ZMM_LAST).contains(&self.into_raw())
    }
}
