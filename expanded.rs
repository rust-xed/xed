#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[macro_use]
mod macros {
    //! This module contains various macros used in the rest of the codebase.
    mod wrapper_enum {
        use std::ffi::c_uint;
        pub(crate) use wrapper_enum;
        pub(crate) const fn is_contiguous(variants: &[c_uint]) -> Option<(c_uint, c_uint)> {
            if variants.is_empty() {
                return None;
            }
            let mut min = variants[0];
            let mut max = variants[0];
            let mut index = 0;
            while index < variants.len() {
                let variant = variants[index];
                if variant < min {
                    min = variant;
                }
                if variant > max {
                    max = variant;
                }
                index += 1;
            }
            let mut variant = min;
            while variant <= max {
                if !contains(variants, variant) {
                    return None;
                }
                variant += 1;
            }
            Some((min, max))
        }
        const fn contains(variants: &[c_uint], value: c_uint) -> bool {
            let mut index = 0;
            while index < variants.len() {
                if variants[index] == value {
                    return true;
                }
                index += 1;
            }
            false
        }
    }
    pub(crate) use wrapper_enum::{is_contiguous, wrapper_enum};
}
pub mod raw {
    //! Low-level bindings for XED.
    //!
    //! These are safe rust bindings for XED that map one-to-one with the XED C API.
    mod address_width {
        use xed_sys::*;
        use crate::macros::wrapper_enum;
        #[repr(u32)]
        pub enum AddressWidth {
            /// 16b addressing
            W16b = XED_ADDRESS_WIDTH_16b as _,
            /// 32b addressing
            W32b = XED_ADDRESS_WIDTH_32b as _,
            /// 64b addressing
            W64b = XED_ADDRESS_WIDTH_64b as _,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for AddressWidth {}
        #[automatically_derived]
        impl ::core::clone::Clone for AddressWidth {
            #[inline]
            fn clone(&self) -> AddressWidth {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AddressWidth {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        AddressWidth::W16b => "W16b",
                        AddressWidth::W32b => "W32b",
                        AddressWidth::W64b => "W64b",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for AddressWidth {}
        #[automatically_derived]
        impl ::core::cmp::Eq for AddressWidth {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for AddressWidth {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for AddressWidth {
            #[inline]
            fn eq(&self, other: &AddressWidth) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for AddressWidth {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        #[automatically_derived]
        impl From<AddressWidth> for core::ffi::c_uint {
            fn from(value: AddressWidth) -> Self {
                value as _
            }
        }
        #[automatically_derived]
        impl core::convert::TryFrom<core::ffi::c_uint> for AddressWidth {
            type Error = crate::error::InvalidEnumValue<core::ffi::c_uint>;
            #[allow(non_upper_case_globals)]
            fn try_from(value: core::ffi::c_uint) -> Result<Self, Self::Error> {
                use core::ffi::c_uint;
                const W16b: c_uint = AddressWidth::W16b as c_uint;
                const W32b: c_uint = AddressWidth::W32b as c_uint;
                const W64b: c_uint = AddressWidth::W64b as c_uint;
                const _CONTIGUOUS: Option<(c_uint, c_uint)> =
                    crate::macros::is_contiguous(&[W16b, W32b, W64b]);
                if let Some((min, max)) = _CONTIGUOUS {
                    if !(min..=max).contains(&value) {
                        return Err(crate::error::InvalidEnumValue::new(value, "AddressWidth"));
                    }
                    Ok(match std::mem::size_of::<AddressWidth>() {
                        1 => unsafe { std::mem::transmute_copy::<_, AddressWidth>(&(value as u8)) },
                        2 => unsafe {
                            std::mem::transmute_copy::<_, AddressWidth>(&(value as u16))
                        },
                        4 => unsafe {
                            std::mem::transmute_copy::<_, AddressWidth>(&(value as u32))
                        },
                        8 => unsafe {
                            std::mem::transmute_copy::<_, AddressWidth>(&(value as u64))
                        },
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    })
                } else {
                    match value {
                        W16b => Ok(Self::W16b),
                        W32b => Ok(Self::W32b),
                        W64b => Ok(Self::W64b),
                        _ => Err(crate::error::InvalidEnumValue::new(value, "AddressWidth")),
                    }
                }
            }
        }
    }
    mod attribute {
        use xed_sys::*;
        use crate::macros::wrapper_enum;
        #[non_exhaustive]
        #[repr(u32)]
        pub enum Attribute {
            AmdOnly = XED_ATTRIBUTE_AMDONLY as _,
            ApxNdd = XED_ATTRIBUTE_APX_NDD as _,
            ApxNf = XED_ATTRIBUTE_APX_NF as _,
            Atomic = XED_ATTRIBUTE_ATOMIC as _,
            AttOperandOrderException = XED_ATTRIBUTE_ATT_OPERAND_ORDER_EXCEPTION as _,
            BroadcastEnabled = XED_ATTRIBUTE_BROADCAST_ENABLED as _,
            ByteOp = XED_ATTRIBUTE_BYTEOP as _,
            Disp8EighthMem = XED_ATTRIBUTE_DISP8_EIGHTHMEM as _,
            Disp8Full = XED_ATTRIBUTE_DISP8_FULL as _,
            Disp8GprReader = XED_ATTRIBUTE_DISP8_GPR_READER as _,
            Disp8GprReaderByte = XED_ATTRIBUTE_DISP8_GPR_READER_BYTE as _,
            Disp8GprReaderWord = XED_ATTRIBUTE_DISP8_GPR_READER_WORD as _,
            Disp8GprWriterLdopD = XED_ATTRIBUTE_DISP8_GPR_WRITER_LDOP_D as _,
            Disp8GprWriterLdopQ = XED_ATTRIBUTE_DISP8_GPR_WRITER_LDOP_Q as _,
            Disp8GprWriterStore = XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE as _,
            Disp8GprWriterStoreByte = XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE_BYTE as _,
            Disp8GprWriterStoreWord = XED_ATTRIBUTE_DISP8_GPR_WRITER_STORE_WORD as _,
            Disp8Gscat = XED_ATTRIBUTE_DISP8_GSCAT as _,
            Disp8Half = XED_ATTRIBUTE_DISP8_HALF as _,
            Disp8HalfMem = XED_ATTRIBUTE_DISP8_HALFMEM as _,
            Disp8Mem128 = XED_ATTRIBUTE_DISP8_MEM128 as _,
            Disp8MovdDup = XED_ATTRIBUTE_DISP8_MOVDDUP as _,
            Disp8NoScale = XED_ATTRIBUTE_DISP8_NO_SCALE as _,
            Disp8Quarter = XED_ATTRIBUTE_DISP8_QUARTER as _,
            Disp8QuarterMem = XED_ATTRIBUTE_DISP8_QUARTERMEM as _,
            Disp8Scalar = XED_ATTRIBUTE_DISP8_SCALAR as _,
            Disp8Tuple1 = XED_ATTRIBUTE_DISP8_TUPLE1 as _,
            Disp8Tuple1_4x = XED_ATTRIBUTE_DISP8_TUPLE1_4X as _,
            Disp8Tuple1Byte = XED_ATTRIBUTE_DISP8_TUPLE1_BYTE as _,
            Disp8Tuple1Word = XED_ATTRIBUTE_DISP8_TUPLE1_WORD as _,
            Disp8Tuple2 = XED_ATTRIBUTE_DISP8_TUPLE2 as _,
            Disp8Tuple4 = XED_ATTRIBUTE_DISP8_TUPLE4 as _,
            Disp8Tuple8 = XED_ATTRIBUTE_DISP8_TUPLE8 as _,
            DoubleWideMemOp = XED_ATTRIBUTE_DOUBLE_WIDE_MEMOP as _,
            DoubleWideOutput = XED_ATTRIBUTE_DOUBLE_WIDE_OUTPUT as _,
            DwordIndices = XED_ATTRIBUTE_DWORD_INDICES as _,
            ElementSizeD = XED_ATTRIBUTE_ELEMENT_SIZE_D as _,
            ElementSizeQ = XED_ATTRIBUTE_ELEMENT_SIZE_Q as _,
            ExceptionBr = XED_ATTRIBUTE_EXCEPTION_BR as _,
            FarXfer = XED_ATTRIBUTE_FAR_XFER as _,
            FixedBase0 = XED_ATTRIBUTE_FIXED_BASE0 as _,
            FixedBase1 = XED_ATTRIBUTE_FIXED_BASE1 as _,
            FixedRoundingRNE = XED_ATTRIBUTE_FIXED_ROUNDING_RNE as _,
            FlushInputDenorm = XED_ATTRIBUTE_FLUSH_INPUT_DENORM as _,
            FlushOutputDenorm = XED_ATTRIBUTE_FLUSH_OUTPUT_DENORM as _,
            Gather = XED_ATTRIBUTE_GATHER as _,
            HalfWideOutput = XED_ATTRIBUTE_HALF_WIDE_OUTPUT as _,
            HleAcqAble = XED_ATTRIBUTE_HLE_ACQ_ABLE as _,
            HleRelAble = XED_ATTRIBUTE_HLE_REL_ABLE as _,
            IgnoresOsfxsr = XED_ATTRIBUTE_IGNORES_OSFXSR as _,
            ImplicitOne = XED_ATTRIBUTE_IMPLICIT_ONE as _,
            IndexRegIsPointer = XED_ATTRIBUTE_INDEX_REG_IS_POINTER as _,
            IndirectBranch = XED_ATTRIBUTE_INDIRECT_BRANCH as _,
            KMask = XED_ATTRIBUTE_KMASK as _,
            Lockable = XED_ATTRIBUTE_LOCKABLE as _,
            Locked = XED_ATTRIBUTE_LOCKED as _,
            MaskOp = XED_ATTRIBUTE_MASKOP as _,
            MaskOpEvex = XED_ATTRIBUTE_MASKOP_EVEX as _,
            MaskAsControl = XED_ATTRIBUTE_MASK_AS_CONTROL as _,
            MaskVariableMemOp = XED_ATTRIBUTE_MASK_VARIABLE_MEMOP as _,
            MemoryFaultSuppression = XED_ATTRIBUTE_MEMORY_FAULT_SUPPRESSION as _,
            MmxExcept = XED_ATTRIBUTE_MMX_EXCEPT as _,
            MpxPrefixAble = XED_ATTRIBUTE_MPX_PREFIX_ABLE as _,
            Multidest2 = XED_ATTRIBUTE_MULTIDEST2 as _,
            MultiSource4 = XED_ATTRIBUTE_MULTISOURCE4 as _,
            Mxcsr = XED_ATTRIBUTE_MXCSR as _,
            MxcsrRd = XED_ATTRIBUTE_MXCSR_RD as _,
            NonTemporal = XED_ATTRIBUTE_NONTEMPORAL as _,
            Nop = XED_ATTRIBUTE_NOP as _,
            NotSx = XED_ATTRIBUTE_NOTSX as _,
            NotSxCond = XED_ATTRIBUTE_NOTSX_COND as _,
            NoRipRel = XED_ATTRIBUTE_NO_RIP_REL as _,
            NoSrcDestMatch = XED_ATTRIBUTE_NO_SRC_DEST_MATCH as _,
            Prefetch = XED_ATTRIBUTE_PREFETCH as _,
            ProtectedMode = XED_ATTRIBUTE_PROTECTED_MODE as _,
            QWordIndices = XED_ATTRIBUTE_QWORD_INDICES as _,
            Rep = XED_ATTRIBUTE_REP as _,
            RequiresAlignment = XED_ATTRIBUTE_REQUIRES_ALIGNMENT as _,
            RequiresAlignment4B = XED_ATTRIBUTE_REQUIRES_ALIGNMENT_4B as _,
            RequiresAlignment8B = XED_ATTRIBUTE_REQUIRES_ALIGNMENT_8B as _,
            Ring0 = XED_ATTRIBUTE_RING0 as _,
            Scalable = XED_ATTRIBUTE_SCALABLE as _,
            Scatter = XED_ATTRIBUTE_SCATTER as _,
            SimdScalar = XED_ATTRIBUTE_SIMD_SCALAR as _,
            SkipLow32 = XED_ATTRIBUTE_SKIPLOW32 as _,
            SkipLow64 = XED_ATTRIBUTE_SKIPLOW64 as _,
            SpecialAgenRequired = XED_ATTRIBUTE_SPECIAL_AGEN_REQUIRED as _,
            StackPop0 = XED_ATTRIBUTE_STACKPOP0 as _,
            StackPop1 = XED_ATTRIBUTE_STACKPOP1 as _,
            StackPush0 = XED_ATTRIBUTE_STACKPUSH0 as _,
            StackPush1 = XED_ATTRIBUTE_STACKPUSH1 as _,
            Undocumented = XED_ATTRIBUTE_UNDOCUMENTED as _,
            UsesDaz = XED_ATTRIBUTE_USES_DAZ as _,
            UsesFtz = XED_ATTRIBUTE_USES_FTZ as _,
            X87Control = XED_ATTRIBUTE_X87_CONTROL as _,
            X87MmxStateCw = XED_ATTRIBUTE_X87_MMX_STATE_CW as _,
            X87MmxStateR = XED_ATTRIBUTE_X87_MMX_STATE_R as _,
            X87MmxStateW = XED_ATTRIBUTE_X87_MMX_STATE_W as _,
            X87NoWait = XED_ATTRIBUTE_X87_NOWAIT as _,
            XmmStateCw = XED_ATTRIBUTE_XMM_STATE_CW as _,
            XmmStateR = XED_ATTRIBUTE_XMM_STATE_R as _,
            XmmStateW = XED_ATTRIBUTE_XMM_STATE_W as _,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Attribute {}
        #[automatically_derived]
        impl ::core::clone::Clone for Attribute {
            #[inline]
            fn clone(&self) -> Attribute {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Attribute {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        Attribute::AmdOnly => "AmdOnly",
                        Attribute::ApxNdd => "ApxNdd",
                        Attribute::ApxNf => "ApxNf",
                        Attribute::Atomic => "Atomic",
                        Attribute::AttOperandOrderException => "AttOperandOrderException",
                        Attribute::BroadcastEnabled => "BroadcastEnabled",
                        Attribute::ByteOp => "ByteOp",
                        Attribute::Disp8EighthMem => "Disp8EighthMem",
                        Attribute::Disp8Full => "Disp8Full",
                        Attribute::Disp8GprReader => "Disp8GprReader",
                        Attribute::Disp8GprReaderByte => "Disp8GprReaderByte",
                        Attribute::Disp8GprReaderWord => "Disp8GprReaderWord",
                        Attribute::Disp8GprWriterLdopD => "Disp8GprWriterLdopD",
                        Attribute::Disp8GprWriterLdopQ => "Disp8GprWriterLdopQ",
                        Attribute::Disp8GprWriterStore => "Disp8GprWriterStore",
                        Attribute::Disp8GprWriterStoreByte => "Disp8GprWriterStoreByte",
                        Attribute::Disp8GprWriterStoreWord => "Disp8GprWriterStoreWord",
                        Attribute::Disp8Gscat => "Disp8Gscat",
                        Attribute::Disp8Half => "Disp8Half",
                        Attribute::Disp8HalfMem => "Disp8HalfMem",
                        Attribute::Disp8Mem128 => "Disp8Mem128",
                        Attribute::Disp8MovdDup => "Disp8MovdDup",
                        Attribute::Disp8NoScale => "Disp8NoScale",
                        Attribute::Disp8Quarter => "Disp8Quarter",
                        Attribute::Disp8QuarterMem => "Disp8QuarterMem",
                        Attribute::Disp8Scalar => "Disp8Scalar",
                        Attribute::Disp8Tuple1 => "Disp8Tuple1",
                        Attribute::Disp8Tuple1_4x => "Disp8Tuple1_4x",
                        Attribute::Disp8Tuple1Byte => "Disp8Tuple1Byte",
                        Attribute::Disp8Tuple1Word => "Disp8Tuple1Word",
                        Attribute::Disp8Tuple2 => "Disp8Tuple2",
                        Attribute::Disp8Tuple4 => "Disp8Tuple4",
                        Attribute::Disp8Tuple8 => "Disp8Tuple8",
                        Attribute::DoubleWideMemOp => "DoubleWideMemOp",
                        Attribute::DoubleWideOutput => "DoubleWideOutput",
                        Attribute::DwordIndices => "DwordIndices",
                        Attribute::ElementSizeD => "ElementSizeD",
                        Attribute::ElementSizeQ => "ElementSizeQ",
                        Attribute::ExceptionBr => "ExceptionBr",
                        Attribute::FarXfer => "FarXfer",
                        Attribute::FixedBase0 => "FixedBase0",
                        Attribute::FixedBase1 => "FixedBase1",
                        Attribute::FixedRoundingRNE => "FixedRoundingRNE",
                        Attribute::FlushInputDenorm => "FlushInputDenorm",
                        Attribute::FlushOutputDenorm => "FlushOutputDenorm",
                        Attribute::Gather => "Gather",
                        Attribute::HalfWideOutput => "HalfWideOutput",
                        Attribute::HleAcqAble => "HleAcqAble",
                        Attribute::HleRelAble => "HleRelAble",
                        Attribute::IgnoresOsfxsr => "IgnoresOsfxsr",
                        Attribute::ImplicitOne => "ImplicitOne",
                        Attribute::IndexRegIsPointer => "IndexRegIsPointer",
                        Attribute::IndirectBranch => "IndirectBranch",
                        Attribute::KMask => "KMask",
                        Attribute::Lockable => "Lockable",
                        Attribute::Locked => "Locked",
                        Attribute::MaskOp => "MaskOp",
                        Attribute::MaskOpEvex => "MaskOpEvex",
                        Attribute::MaskAsControl => "MaskAsControl",
                        Attribute::MaskVariableMemOp => "MaskVariableMemOp",
                        Attribute::MemoryFaultSuppression => "MemoryFaultSuppression",
                        Attribute::MmxExcept => "MmxExcept",
                        Attribute::MpxPrefixAble => "MpxPrefixAble",
                        Attribute::Multidest2 => "Multidest2",
                        Attribute::MultiSource4 => "MultiSource4",
                        Attribute::Mxcsr => "Mxcsr",
                        Attribute::MxcsrRd => "MxcsrRd",
                        Attribute::NonTemporal => "NonTemporal",
                        Attribute::Nop => "Nop",
                        Attribute::NotSx => "NotSx",
                        Attribute::NotSxCond => "NotSxCond",
                        Attribute::NoRipRel => "NoRipRel",
                        Attribute::NoSrcDestMatch => "NoSrcDestMatch",
                        Attribute::Prefetch => "Prefetch",
                        Attribute::ProtectedMode => "ProtectedMode",
                        Attribute::QWordIndices => "QWordIndices",
                        Attribute::Rep => "Rep",
                        Attribute::RequiresAlignment => "RequiresAlignment",
                        Attribute::RequiresAlignment4B => "RequiresAlignment4B",
                        Attribute::RequiresAlignment8B => "RequiresAlignment8B",
                        Attribute::Ring0 => "Ring0",
                        Attribute::Scalable => "Scalable",
                        Attribute::Scatter => "Scatter",
                        Attribute::SimdScalar => "SimdScalar",
                        Attribute::SkipLow32 => "SkipLow32",
                        Attribute::SkipLow64 => "SkipLow64",
                        Attribute::SpecialAgenRequired => "SpecialAgenRequired",
                        Attribute::StackPop0 => "StackPop0",
                        Attribute::StackPop1 => "StackPop1",
                        Attribute::StackPush0 => "StackPush0",
                        Attribute::StackPush1 => "StackPush1",
                        Attribute::Undocumented => "Undocumented",
                        Attribute::UsesDaz => "UsesDaz",
                        Attribute::UsesFtz => "UsesFtz",
                        Attribute::X87Control => "X87Control",
                        Attribute::X87MmxStateCw => "X87MmxStateCw",
                        Attribute::X87MmxStateR => "X87MmxStateR",
                        Attribute::X87MmxStateW => "X87MmxStateW",
                        Attribute::X87NoWait => "X87NoWait",
                        Attribute::XmmStateCw => "XmmStateCw",
                        Attribute::XmmStateR => "XmmStateR",
                        Attribute::XmmStateW => "XmmStateW",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Attribute {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Attribute {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Attribute {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Attribute {
            #[inline]
            fn eq(&self, other: &Attribute) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Attribute {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        #[automatically_derived]
        impl From<Attribute> for core::ffi::c_uint {
            fn from(value: Attribute) -> Self {
                value as _
            }
        }
        #[automatically_derived]
        impl core::convert::TryFrom<core::ffi::c_uint> for Attribute {
            type Error = crate::error::InvalidEnumValue<core::ffi::c_uint>;
            #[allow(non_upper_case_globals)]
            fn try_from(value: core::ffi::c_uint) -> Result<Self, Self::Error> {
                use core::ffi::c_uint;
                const AmdOnly: c_uint = Attribute::AmdOnly as c_uint;
                const ApxNdd: c_uint = Attribute::ApxNdd as c_uint;
                const ApxNf: c_uint = Attribute::ApxNf as c_uint;
                const Atomic: c_uint = Attribute::Atomic as c_uint;
                const AttOperandOrderException: c_uint =
                    Attribute::AttOperandOrderException as c_uint;
                const BroadcastEnabled: c_uint = Attribute::BroadcastEnabled as c_uint;
                const ByteOp: c_uint = Attribute::ByteOp as c_uint;
                const Disp8EighthMem: c_uint = Attribute::Disp8EighthMem as c_uint;
                const Disp8Full: c_uint = Attribute::Disp8Full as c_uint;
                const Disp8GprReader: c_uint = Attribute::Disp8GprReader as c_uint;
                const Disp8GprReaderByte: c_uint = Attribute::Disp8GprReaderByte as c_uint;
                const Disp8GprReaderWord: c_uint = Attribute::Disp8GprReaderWord as c_uint;
                const Disp8GprWriterLdopD: c_uint = Attribute::Disp8GprWriterLdopD as c_uint;
                const Disp8GprWriterLdopQ: c_uint = Attribute::Disp8GprWriterLdopQ as c_uint;
                const Disp8GprWriterStore: c_uint = Attribute::Disp8GprWriterStore as c_uint;
                const Disp8GprWriterStoreByte: c_uint =
                    Attribute::Disp8GprWriterStoreByte as c_uint;
                const Disp8GprWriterStoreWord: c_uint =
                    Attribute::Disp8GprWriterStoreWord as c_uint;
                const Disp8Gscat: c_uint = Attribute::Disp8Gscat as c_uint;
                const Disp8Half: c_uint = Attribute::Disp8Half as c_uint;
                const Disp8HalfMem: c_uint = Attribute::Disp8HalfMem as c_uint;
                const Disp8Mem128: c_uint = Attribute::Disp8Mem128 as c_uint;
                const Disp8MovdDup: c_uint = Attribute::Disp8MovdDup as c_uint;
                const Disp8NoScale: c_uint = Attribute::Disp8NoScale as c_uint;
                const Disp8Quarter: c_uint = Attribute::Disp8Quarter as c_uint;
                const Disp8QuarterMem: c_uint = Attribute::Disp8QuarterMem as c_uint;
                const Disp8Scalar: c_uint = Attribute::Disp8Scalar as c_uint;
                const Disp8Tuple1: c_uint = Attribute::Disp8Tuple1 as c_uint;
                const Disp8Tuple1_4x: c_uint = Attribute::Disp8Tuple1_4x as c_uint;
                const Disp8Tuple1Byte: c_uint = Attribute::Disp8Tuple1Byte as c_uint;
                const Disp8Tuple1Word: c_uint = Attribute::Disp8Tuple1Word as c_uint;
                const Disp8Tuple2: c_uint = Attribute::Disp8Tuple2 as c_uint;
                const Disp8Tuple4: c_uint = Attribute::Disp8Tuple4 as c_uint;
                const Disp8Tuple8: c_uint = Attribute::Disp8Tuple8 as c_uint;
                const DoubleWideMemOp: c_uint = Attribute::DoubleWideMemOp as c_uint;
                const DoubleWideOutput: c_uint = Attribute::DoubleWideOutput as c_uint;
                const DwordIndices: c_uint = Attribute::DwordIndices as c_uint;
                const ElementSizeD: c_uint = Attribute::ElementSizeD as c_uint;
                const ElementSizeQ: c_uint = Attribute::ElementSizeQ as c_uint;
                const ExceptionBr: c_uint = Attribute::ExceptionBr as c_uint;
                const FarXfer: c_uint = Attribute::FarXfer as c_uint;
                const FixedBase0: c_uint = Attribute::FixedBase0 as c_uint;
                const FixedBase1: c_uint = Attribute::FixedBase1 as c_uint;
                const FixedRoundingRNE: c_uint = Attribute::FixedRoundingRNE as c_uint;
                const FlushInputDenorm: c_uint = Attribute::FlushInputDenorm as c_uint;
                const FlushOutputDenorm: c_uint = Attribute::FlushOutputDenorm as c_uint;
                const Gather: c_uint = Attribute::Gather as c_uint;
                const HalfWideOutput: c_uint = Attribute::HalfWideOutput as c_uint;
                const HleAcqAble: c_uint = Attribute::HleAcqAble as c_uint;
                const HleRelAble: c_uint = Attribute::HleRelAble as c_uint;
                const IgnoresOsfxsr: c_uint = Attribute::IgnoresOsfxsr as c_uint;
                const ImplicitOne: c_uint = Attribute::ImplicitOne as c_uint;
                const IndexRegIsPointer: c_uint = Attribute::IndexRegIsPointer as c_uint;
                const IndirectBranch: c_uint = Attribute::IndirectBranch as c_uint;
                const KMask: c_uint = Attribute::KMask as c_uint;
                const Lockable: c_uint = Attribute::Lockable as c_uint;
                const Locked: c_uint = Attribute::Locked as c_uint;
                const MaskOp: c_uint = Attribute::MaskOp as c_uint;
                const MaskOpEvex: c_uint = Attribute::MaskOpEvex as c_uint;
                const MaskAsControl: c_uint = Attribute::MaskAsControl as c_uint;
                const MaskVariableMemOp: c_uint = Attribute::MaskVariableMemOp as c_uint;
                const MemoryFaultSuppression: c_uint = Attribute::MemoryFaultSuppression as c_uint;
                const MmxExcept: c_uint = Attribute::MmxExcept as c_uint;
                const MpxPrefixAble: c_uint = Attribute::MpxPrefixAble as c_uint;
                const Multidest2: c_uint = Attribute::Multidest2 as c_uint;
                const MultiSource4: c_uint = Attribute::MultiSource4 as c_uint;
                const Mxcsr: c_uint = Attribute::Mxcsr as c_uint;
                const MxcsrRd: c_uint = Attribute::MxcsrRd as c_uint;
                const NonTemporal: c_uint = Attribute::NonTemporal as c_uint;
                const Nop: c_uint = Attribute::Nop as c_uint;
                const NotSx: c_uint = Attribute::NotSx as c_uint;
                const NotSxCond: c_uint = Attribute::NotSxCond as c_uint;
                const NoRipRel: c_uint = Attribute::NoRipRel as c_uint;
                const NoSrcDestMatch: c_uint = Attribute::NoSrcDestMatch as c_uint;
                const Prefetch: c_uint = Attribute::Prefetch as c_uint;
                const ProtectedMode: c_uint = Attribute::ProtectedMode as c_uint;
                const QWordIndices: c_uint = Attribute::QWordIndices as c_uint;
                const Rep: c_uint = Attribute::Rep as c_uint;
                const RequiresAlignment: c_uint = Attribute::RequiresAlignment as c_uint;
                const RequiresAlignment4B: c_uint = Attribute::RequiresAlignment4B as c_uint;
                const RequiresAlignment8B: c_uint = Attribute::RequiresAlignment8B as c_uint;
                const Ring0: c_uint = Attribute::Ring0 as c_uint;
                const Scalable: c_uint = Attribute::Scalable as c_uint;
                const Scatter: c_uint = Attribute::Scatter as c_uint;
                const SimdScalar: c_uint = Attribute::SimdScalar as c_uint;
                const SkipLow32: c_uint = Attribute::SkipLow32 as c_uint;
                const SkipLow64: c_uint = Attribute::SkipLow64 as c_uint;
                const SpecialAgenRequired: c_uint = Attribute::SpecialAgenRequired as c_uint;
                const StackPop0: c_uint = Attribute::StackPop0 as c_uint;
                const StackPop1: c_uint = Attribute::StackPop1 as c_uint;
                const StackPush0: c_uint = Attribute::StackPush0 as c_uint;
                const StackPush1: c_uint = Attribute::StackPush1 as c_uint;
                const Undocumented: c_uint = Attribute::Undocumented as c_uint;
                const UsesDaz: c_uint = Attribute::UsesDaz as c_uint;
                const UsesFtz: c_uint = Attribute::UsesFtz as c_uint;
                const X87Control: c_uint = Attribute::X87Control as c_uint;
                const X87MmxStateCw: c_uint = Attribute::X87MmxStateCw as c_uint;
                const X87MmxStateR: c_uint = Attribute::X87MmxStateR as c_uint;
                const X87MmxStateW: c_uint = Attribute::X87MmxStateW as c_uint;
                const X87NoWait: c_uint = Attribute::X87NoWait as c_uint;
                const XmmStateCw: c_uint = Attribute::XmmStateCw as c_uint;
                const XmmStateR: c_uint = Attribute::XmmStateR as c_uint;
                const XmmStateW: c_uint = Attribute::XmmStateW as c_uint;
                const _CONTIGUOUS: Option<(c_uint, c_uint)> = crate::macros::is_contiguous(&[
                    AmdOnly,
                    ApxNdd,
                    ApxNf,
                    Atomic,
                    AttOperandOrderException,
                    BroadcastEnabled,
                    ByteOp,
                    Disp8EighthMem,
                    Disp8Full,
                    Disp8GprReader,
                    Disp8GprReaderByte,
                    Disp8GprReaderWord,
                    Disp8GprWriterLdopD,
                    Disp8GprWriterLdopQ,
                    Disp8GprWriterStore,
                    Disp8GprWriterStoreByte,
                    Disp8GprWriterStoreWord,
                    Disp8Gscat,
                    Disp8Half,
                    Disp8HalfMem,
                    Disp8Mem128,
                    Disp8MovdDup,
                    Disp8NoScale,
                    Disp8Quarter,
                    Disp8QuarterMem,
                    Disp8Scalar,
                    Disp8Tuple1,
                    Disp8Tuple1_4x,
                    Disp8Tuple1Byte,
                    Disp8Tuple1Word,
                    Disp8Tuple2,
                    Disp8Tuple4,
                    Disp8Tuple8,
                    DoubleWideMemOp,
                    DoubleWideOutput,
                    DwordIndices,
                    ElementSizeD,
                    ElementSizeQ,
                    ExceptionBr,
                    FarXfer,
                    FixedBase0,
                    FixedBase1,
                    FixedRoundingRNE,
                    FlushInputDenorm,
                    FlushOutputDenorm,
                    Gather,
                    HalfWideOutput,
                    HleAcqAble,
                    HleRelAble,
                    IgnoresOsfxsr,
                    ImplicitOne,
                    IndexRegIsPointer,
                    IndirectBranch,
                    KMask,
                    Lockable,
                    Locked,
                    MaskOp,
                    MaskOpEvex,
                    MaskAsControl,
                    MaskVariableMemOp,
                    MemoryFaultSuppression,
                    MmxExcept,
                    MpxPrefixAble,
                    Multidest2,
                    MultiSource4,
                    Mxcsr,
                    MxcsrRd,
                    NonTemporal,
                    Nop,
                    NotSx,
                    NotSxCond,
                    NoRipRel,
                    NoSrcDestMatch,
                    Prefetch,
                    ProtectedMode,
                    QWordIndices,
                    Rep,
                    RequiresAlignment,
                    RequiresAlignment4B,
                    RequiresAlignment8B,
                    Ring0,
                    Scalable,
                    Scatter,
                    SimdScalar,
                    SkipLow32,
                    SkipLow64,
                    SpecialAgenRequired,
                    StackPop0,
                    StackPop1,
                    StackPush0,
                    StackPush1,
                    Undocumented,
                    UsesDaz,
                    UsesFtz,
                    X87Control,
                    X87MmxStateCw,
                    X87MmxStateR,
                    X87MmxStateW,
                    X87NoWait,
                    XmmStateCw,
                    XmmStateR,
                    XmmStateW,
                ]);
                if let Some((min, max)) = _CONTIGUOUS {
                    if !(min..=max).contains(&value) {
                        return Err(crate::error::InvalidEnumValue::new(value, "Attribute"));
                    }
                    Ok(match std::mem::size_of::<Attribute>() {
                        1 => unsafe { std::mem::transmute_copy::<_, Attribute>(&(value as u8)) },
                        2 => unsafe { std::mem::transmute_copy::<_, Attribute>(&(value as u16)) },
                        4 => unsafe { std::mem::transmute_copy::<_, Attribute>(&(value as u32)) },
                        8 => unsafe { std::mem::transmute_copy::<_, Attribute>(&(value as u64)) },
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    })
                } else {
                    match value {
                        AmdOnly => Ok(Self::AmdOnly),
                        ApxNdd => Ok(Self::ApxNdd),
                        ApxNf => Ok(Self::ApxNf),
                        Atomic => Ok(Self::Atomic),
                        AttOperandOrderException => Ok(Self::AttOperandOrderException),
                        BroadcastEnabled => Ok(Self::BroadcastEnabled),
                        ByteOp => Ok(Self::ByteOp),
                        Disp8EighthMem => Ok(Self::Disp8EighthMem),
                        Disp8Full => Ok(Self::Disp8Full),
                        Disp8GprReader => Ok(Self::Disp8GprReader),
                        Disp8GprReaderByte => Ok(Self::Disp8GprReaderByte),
                        Disp8GprReaderWord => Ok(Self::Disp8GprReaderWord),
                        Disp8GprWriterLdopD => Ok(Self::Disp8GprWriterLdopD),
                        Disp8GprWriterLdopQ => Ok(Self::Disp8GprWriterLdopQ),
                        Disp8GprWriterStore => Ok(Self::Disp8GprWriterStore),
                        Disp8GprWriterStoreByte => Ok(Self::Disp8GprWriterStoreByte),
                        Disp8GprWriterStoreWord => Ok(Self::Disp8GprWriterStoreWord),
                        Disp8Gscat => Ok(Self::Disp8Gscat),
                        Disp8Half => Ok(Self::Disp8Half),
                        Disp8HalfMem => Ok(Self::Disp8HalfMem),
                        Disp8Mem128 => Ok(Self::Disp8Mem128),
                        Disp8MovdDup => Ok(Self::Disp8MovdDup),
                        Disp8NoScale => Ok(Self::Disp8NoScale),
                        Disp8Quarter => Ok(Self::Disp8Quarter),
                        Disp8QuarterMem => Ok(Self::Disp8QuarterMem),
                        Disp8Scalar => Ok(Self::Disp8Scalar),
                        Disp8Tuple1 => Ok(Self::Disp8Tuple1),
                        Disp8Tuple1_4x => Ok(Self::Disp8Tuple1_4x),
                        Disp8Tuple1Byte => Ok(Self::Disp8Tuple1Byte),
                        Disp8Tuple1Word => Ok(Self::Disp8Tuple1Word),
                        Disp8Tuple2 => Ok(Self::Disp8Tuple2),
                        Disp8Tuple4 => Ok(Self::Disp8Tuple4),
                        Disp8Tuple8 => Ok(Self::Disp8Tuple8),
                        DoubleWideMemOp => Ok(Self::DoubleWideMemOp),
                        DoubleWideOutput => Ok(Self::DoubleWideOutput),
                        DwordIndices => Ok(Self::DwordIndices),
                        ElementSizeD => Ok(Self::ElementSizeD),
                        ElementSizeQ => Ok(Self::ElementSizeQ),
                        ExceptionBr => Ok(Self::ExceptionBr),
                        FarXfer => Ok(Self::FarXfer),
                        FixedBase0 => Ok(Self::FixedBase0),
                        FixedBase1 => Ok(Self::FixedBase1),
                        FixedRoundingRNE => Ok(Self::FixedRoundingRNE),
                        FlushInputDenorm => Ok(Self::FlushInputDenorm),
                        FlushOutputDenorm => Ok(Self::FlushOutputDenorm),
                        Gather => Ok(Self::Gather),
                        HalfWideOutput => Ok(Self::HalfWideOutput),
                        HleAcqAble => Ok(Self::HleAcqAble),
                        HleRelAble => Ok(Self::HleRelAble),
                        IgnoresOsfxsr => Ok(Self::IgnoresOsfxsr),
                        ImplicitOne => Ok(Self::ImplicitOne),
                        IndexRegIsPointer => Ok(Self::IndexRegIsPointer),
                        IndirectBranch => Ok(Self::IndirectBranch),
                        KMask => Ok(Self::KMask),
                        Lockable => Ok(Self::Lockable),
                        Locked => Ok(Self::Locked),
                        MaskOp => Ok(Self::MaskOp),
                        MaskOpEvex => Ok(Self::MaskOpEvex),
                        MaskAsControl => Ok(Self::MaskAsControl),
                        MaskVariableMemOp => Ok(Self::MaskVariableMemOp),
                        MemoryFaultSuppression => Ok(Self::MemoryFaultSuppression),
                        MmxExcept => Ok(Self::MmxExcept),
                        MpxPrefixAble => Ok(Self::MpxPrefixAble),
                        Multidest2 => Ok(Self::Multidest2),
                        MultiSource4 => Ok(Self::MultiSource4),
                        Mxcsr => Ok(Self::Mxcsr),
                        MxcsrRd => Ok(Self::MxcsrRd),
                        NonTemporal => Ok(Self::NonTemporal),
                        Nop => Ok(Self::Nop),
                        NotSx => Ok(Self::NotSx),
                        NotSxCond => Ok(Self::NotSxCond),
                        NoRipRel => Ok(Self::NoRipRel),
                        NoSrcDestMatch => Ok(Self::NoSrcDestMatch),
                        Prefetch => Ok(Self::Prefetch),
                        ProtectedMode => Ok(Self::ProtectedMode),
                        QWordIndices => Ok(Self::QWordIndices),
                        Rep => Ok(Self::Rep),
                        RequiresAlignment => Ok(Self::RequiresAlignment),
                        RequiresAlignment4B => Ok(Self::RequiresAlignment4B),
                        RequiresAlignment8B => Ok(Self::RequiresAlignment8B),
                        Ring0 => Ok(Self::Ring0),
                        Scalable => Ok(Self::Scalable),
                        Scatter => Ok(Self::Scatter),
                        SimdScalar => Ok(Self::SimdScalar),
                        SkipLow32 => Ok(Self::SkipLow32),
                        SkipLow64 => Ok(Self::SkipLow64),
                        SpecialAgenRequired => Ok(Self::SpecialAgenRequired),
                        StackPop0 => Ok(Self::StackPop0),
                        StackPop1 => Ok(Self::StackPop1),
                        StackPush0 => Ok(Self::StackPush0),
                        StackPush1 => Ok(Self::StackPush1),
                        Undocumented => Ok(Self::Undocumented),
                        UsesDaz => Ok(Self::UsesDaz),
                        UsesFtz => Ok(Self::UsesFtz),
                        X87Control => Ok(Self::X87Control),
                        X87MmxStateCw => Ok(Self::X87MmxStateCw),
                        X87MmxStateR => Ok(Self::X87MmxStateR),
                        X87MmxStateW => Ok(Self::X87MmxStateW),
                        X87NoWait => Ok(Self::X87NoWait),
                        XmmStateCw => Ok(Self::XmmStateCw),
                        XmmStateR => Ok(Self::XmmStateR),
                        XmmStateW => Ok(Self::XmmStateW),
                        _ => Err(crate::error::InvalidEnumValue::new(value, "Attribute")),
                    }
                }
            }
        }
    }
    mod decoded_inst {
        use xed_sys::*;
        pub struct DecodedInst {
            inst: xed_decoded_inst_t,
        }
    }
    mod machine_mode {
        use xed_sys::*;
        use crate::macros::wrapper_enum;
        #[repr(u32)]
        pub enum MachineMode {
            /// 64b operating mode.
            Long64 = XED_MACHINE_MODE_LONG_64 as _,
            /// 32b protected mode.
            LongCompat32 = XED_MACHINE_MODE_LONG_COMPAT_32 as _,
            /// 16b protected mode.
            LongCompat16 = XED_MACHINE_MODE_LONG_COMPAT_16 as _,
            /// 32b protected mode.
            Legacy32 = XED_MACHINE_MODE_LEGACY_32 as _,
            /// 16b protected mode.
            Legacy16 = XED_MACHINE_MODE_LEGACY_16 as _,
            /// 16b real mode.
            Read16 = XED_MACHINE_MODE_REAL_16 as _,
            /// 32b real mode.
            Real32 = XED_MACHINE_MODE_REAL_32 as _,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for MachineMode {}
        #[automatically_derived]
        impl ::core::clone::Clone for MachineMode {
            #[inline]
            fn clone(&self) -> MachineMode {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MachineMode {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        MachineMode::Long64 => "Long64",
                        MachineMode::LongCompat32 => "LongCompat32",
                        MachineMode::LongCompat16 => "LongCompat16",
                        MachineMode::Legacy32 => "Legacy32",
                        MachineMode::Legacy16 => "Legacy16",
                        MachineMode::Read16 => "Read16",
                        MachineMode::Real32 => "Real32",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for MachineMode {}
        #[automatically_derived]
        impl ::core::cmp::Eq for MachineMode {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MachineMode {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MachineMode {
            #[inline]
            fn eq(&self, other: &MachineMode) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for MachineMode {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_tag, state)
            }
        }
        #[automatically_derived]
        impl From<MachineMode> for core::ffi::c_uint {
            fn from(value: MachineMode) -> Self {
                value as _
            }
        }
        #[automatically_derived]
        impl core::convert::TryFrom<core::ffi::c_uint> for MachineMode {
            type Error = crate::error::InvalidEnumValue<core::ffi::c_uint>;
            #[allow(non_upper_case_globals)]
            fn try_from(value: core::ffi::c_uint) -> Result<Self, Self::Error> {
                use core::ffi::c_uint;
                const Long64: c_uint = MachineMode::Long64 as c_uint;
                const LongCompat32: c_uint = MachineMode::LongCompat32 as c_uint;
                const LongCompat16: c_uint = MachineMode::LongCompat16 as c_uint;
                const Legacy32: c_uint = MachineMode::Legacy32 as c_uint;
                const Legacy16: c_uint = MachineMode::Legacy16 as c_uint;
                const Read16: c_uint = MachineMode::Read16 as c_uint;
                const Real32: c_uint = MachineMode::Real32 as c_uint;
                const _CONTIGUOUS: Option<(c_uint, c_uint)> = crate::macros::is_contiguous(&[
                    Long64,
                    LongCompat32,
                    LongCompat16,
                    Legacy32,
                    Legacy16,
                    Read16,
                    Real32,
                ]);
                if let Some((min, max)) = _CONTIGUOUS {
                    if !(min..=max).contains(&value) {
                        return Err(crate::error::InvalidEnumValue::new(value, "MachineMode"));
                    }
                    Ok(match std::mem::size_of::<MachineMode>() {
                        1 => unsafe { std::mem::transmute_copy::<_, MachineMode>(&(value as u8)) },
                        2 => unsafe { std::mem::transmute_copy::<_, MachineMode>(&(value as u16)) },
                        4 => unsafe { std::mem::transmute_copy::<_, MachineMode>(&(value as u32)) },
                        8 => unsafe { std::mem::transmute_copy::<_, MachineMode>(&(value as u64)) },
                        _ => ::core::panicking::panic("internal error: entered unreachable code"),
                    })
                } else {
                    match value {
                        Long64 => Ok(Self::Long64),
                        LongCompat32 => Ok(Self::LongCompat32),
                        LongCompat16 => Ok(Self::LongCompat16),
                        Legacy32 => Ok(Self::Legacy32),
                        Legacy16 => Ok(Self::Legacy16),
                        Read16 => Ok(Self::Read16),
                        Real32 => Ok(Self::Real32),
                        _ => Err(crate::error::InvalidEnumValue::new(value, "MachineMode")),
                    }
                }
            }
        }
    }
    mod state {
        use std::fmt;
        use xed_sys::*;
        use super::{AddressWidth, MachineMode};
        pub struct State(xed_state_t);
        #[automatically_derived]
        impl ::core::marker::Copy for State {}
        #[automatically_derived]
        impl ::core::clone::Clone for State {
            #[inline]
            fn clone(&self) -> State {
                let _: ::core::clone::AssertParamIsClone<xed_state_t>;
                *self
            }
        }
        impl State {
            /// Create a new state from a machine mode and stack address width.
            ///
            /// # Panics
            /// - If `mmode` is [`MachineMode::Long64`] and `stack_addr_width` is not
            ///   [`AddressWidth::W64b`].
            pub fn new(mmode: MachineMode, stack_addr_width: AddressWidth) -> Self {
                if mmode == MachineMode::Long64 {
                    match (&stack_addr_width, &AddressWidth::W64b) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    };
                }
                crate::raw::init_tables();
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
    }
    pub use self::address_width::AddressWidth;
    pub use self::attribute::Attribute;
    pub use self::decoded_inst::DecodedInst;
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
        TABLES_INIT.get_or_init(|| unsafe { xed_sys::xed_tables_init() });
    }
}
pub mod error {
    //! Various error types used within `xed`.
    use core::fmt;
    /// Error for when converting an integer to an enum fails.
    pub struct InvalidEnumValue<T> {
        value: T,
        name: &'static str,
    }
    #[automatically_derived]
    impl<T: ::core::marker::Copy> ::core::marker::Copy for InvalidEnumValue<T> {}
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for InvalidEnumValue<T> {
        #[inline]
        fn clone(&self) -> InvalidEnumValue<T> {
            InvalidEnumValue {
                value: ::core::clone::Clone::clone(&self.value),
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for InvalidEnumValue<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "InvalidEnumValue",
                "value",
                &self.value,
                "name",
                &&self.name,
            )
        }
    }
    impl<T> InvalidEnumValue<T> {
        pub(crate) fn new(value: T, name: &'static str) -> Self {
            Self { value, name }
        }
        pub fn value(self) -> T {
            self.value
        }
        pub fn enum_name(&self) -> &str {
            self.name
        }
    }
    impl<T: fmt::Debug> fmt::Display for InvalidEnumValue<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_fmt(format_args!(
                "{0:?} is not a valid value for {1}",
                self.value, self.name
            ))
        }
    }
}
