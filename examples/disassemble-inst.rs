use anyhow::Context;
use clap::Parser;
use common::*;

mod common;

/// Disassemble bytes (in hex) into assembly.
#[derive(Debug, clap::Parser)]
struct Args {
    /// The machine mode to assume when decoding the instruction.
    #[arg(long, value_enum, default_value_t = MachineMode::Long64)]
    pub mode: MachineMode,

    /// The address width to assume when decoding the instruction.
    #[arg(long, value_enum, default_value_t = AddressWidth::QWord)]
    pub width: AddressWidth,

    /// The disassembly syntax to use.
    #[arg(long, value_enum, default_value_t = Syntax::Intel)]
    pub syntax: Syntax,

    /// The bytes to decode, hex-encoded.
    pub bytes: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let state = xed::State::new(args.mode.into_xed(), args.width.into_xed());
    let bytes = hex::decode(&args.bytes)?;

    let opts = xed::DecodeOptions::new(state);
    let inst = xed::decode(&bytes, opts).context("failed to decode the instruction")?;
    let disassembly = inst.disassemble(args.syntax.into_xed());

    println!("{disassembly}");

    Ok(())
}
