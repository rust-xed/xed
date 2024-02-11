//! Common helpers that are shared between various examples.
//!
//! These are just variants of various XED enums that can be used with clap.

use clap::builder::{StringValueParser, TypedValueParser};
use clap::error::ErrorKind;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, clap::ValueEnum)]
pub enum MachineMode {
    Long64,
    LongCompat32,
    LongCompat16,
    Legacy32,
    Legacy16,
    Real16,
    Real32,
}

impl MachineMode {
    pub fn into_xed(self) -> xed::MachineMode {
        use xed::MachineMode::*;

        match self {
            Self::Long64 => Long64,
            Self::LongCompat32 => LongCompat32,
            Self::LongCompat16 => LongCompat16,
            Self::Legacy32 => Legacy32,
            Self::Legacy16 => Legacy16,
            Self::Real32 => Real32,
            Self::Real16 => Real16,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum AddressWidth {
    Word,
    DWord,
    QWord,
}

impl clap::ValueEnum for AddressWidth {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Word, Self::DWord, Self::QWord]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;

        Some(match self {
            Self::Word => PossibleValue::new("16"),
            Self::DWord => PossibleValue::new("32"),
            Self::QWord => PossibleValue::new("64"),
        })
    }
}

impl AddressWidth {
    pub fn into_xed(self) -> xed::AddressWidth {
        use xed::AddressWidth::*;

        match self {
            Self::Word => Word,
            Self::QWord => QWord,
            Self::DWord => DWord,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, clap::ValueEnum)]
pub enum Syntax {
    Xed,
    Att,
    Intel,
}

impl Syntax {
    pub fn into_xed(self) -> xed::Syntax {
        match self {
            Self::Xed => xed::Syntax::XED,
            Self::Att => xed::Syntax::ATT,
            Self::Intel => xed::Syntax::INTEL,
        }
    }
}

#[derive(Clone, Copy)]
pub struct HexValueParser;

impl TypedValueParser for HexValueParser {
    type Value = Vec<u8>;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let text = StringValueParser::new().parse_ref(cmd, arg, value)?;
        let bytes = match hex::decode(text) {
            Ok(bytes) => bytes,
            Err(e) => return Err(clap::Error::raw(ErrorKind::ValueValidation, e)),
        };

        Ok(bytes)
    }
}
