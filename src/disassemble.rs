//! Instruction disassembly.
//!
//! This module contains helpers for disassembling instructions via
//! [`DecodedInst::disassemble`], [`DecodedInst::disassemble_with`], and
//! [`DecodedInst::try_disassemble_with`].
//!
//! Simple use cases can get away with just using [`DecodedInst::disassemble`]
//! with the desired syntax. For more complicated uses see the examples below as
//! well as the options on [`DisassembleOptions`].
//!
//! # Examples
//! Disassemble an instruction to both intel and AT&T syntax.
//! ```
//! let bytes = [0xF3, 0x0F, 0xBD, 0x17];
//! let state = xed::State::long64();
//! let options = xed::DecodeOptions::new(state);
//! let inst = xed::decode(&bytes, options)?;
//!
//! println!("{}", inst.disassemble(xed::Syntax::INTEL)); // lzcnt edx, dword ptr [rdi]
//! println!("{}", inst.disassemble(xed::Syntax::ATT));   // lzcntl (%rdi), %edx
//! #
//! # Result::<_, xed::Error>::Ok(())
//! ```
//!
//! Disassemble an instruction with some basic symbol resolution.
//! ```
//! use xed::disassemble::{DisassembleOptions, WriteBuf};
//!
//! let bytes = [0xE9, 0x80, 0x01, 0x00, 0x00];
//! let state = xed::State::long64();
//! let options = xed::DecodeOptions::new(state);
//! let inst = xed::decode(&bytes, options)?;
//!
//! let options = DisassembleOptions::intel()
//!     .runtime_address(0x1000)
//!     .callback(|address, buf: &mut WriteBuf<'_>| {
//!         // We resolve everything above 0x1100 to the symbol `execute`
//!         (address >= 0x1100).then_some(address - 0x1100)
//!     });
//!
//! // jmp  <execute+0x85>
//! println!("{}", inst.disassemble_with(&options));
//! #
//! # Result::<_, xed::Error>::Ok(())
//! ```
//!
//! Disassemble an instruction with fallible symbol resolution
//! ```
//! use xed::disassemble::{DisassembleOptions, DisassemblyCallback, WriteBuf};
//!
//! let bytes = [0xE9, 0x80, 0x01, 0x00, 0x00];
//! let state = xed::State::long64();
//! let options = xed::DecodeOptions::new(state);
//! let inst = xed::decode(&bytes, options)?;
//!
//! struct ResolveCallback;
//!
//! impl DisassemblyCallback for ResolveCallback {
//!     type Error = &'static str;
//!
//!     fn resolve(
//!         &self,
//!         address: u64,
//!         symbol: &mut WriteBuf<'_>
//!     ) -> Result<Option<u64>, &'static str> {
//!         if address & 1 != 0 {
//!             return Err("we don't resolve odd numbered addresses");
//!         }
//!
//!         Ok((address >= 0x1100).then_some(address - 0x1100))
//!     }
//! }
//!
//! let options = DisassembleOptions::intel()
//!     .runtime_address(0x1000)
//!     .callback(ResolveCallback);
//!
//! inst.try_disassemble_with(&options)
//!     .expect_err("jump target should have been odd");
//! #
//! # Result::<_, xed::Error>::Ok(())
//! ```

use std::any::Any;
use std::borrow::Cow;
use std::convert::Infallible;
use std::ffi::{c_char, c_int, c_void};
use std::fmt;
use std::mem::MaybeUninit;
use std::panic::AssertUnwindSafe;

use xed_sys::*;

use crate::{DecodedInst, Syntax};

struct CallbackContext<'a, F, E> {
    cb: &'a F,
    error: Option<E>,
    panic: Option<Box<dyn Any + Send>>,
}

/// Wrapper callback used for xed_format_generic.
unsafe extern "C" fn disassembly_callback<F>(
    address: u64,
    symbol_buf: *mut c_char,
    buflen: u32,
    offset: *mut u64,
    context: *mut c_void,
) -> c_int
where
    F: DisassemblyCallback,
{
    let context = unsafe { &mut *(context as *mut CallbackContext<F, F::Error>) };
    let buffer = unsafe {
        std::slice::from_raw_parts_mut(symbol_buf as *mut MaybeUninit<u8>, buflen as usize)
    };
    let offset = unsafe { &mut *offset };

    // If a callback has already panicked then we don't want to run any more
    // callbacks. Return a resolve failure and then we'll re-raise the panic at the
    // top level.
    //
    // This also protects against the edge case where the drop impl of the panic
    // payload itself panics.
    if context.panic.is_some() {
        return 0;
    }

    // There's already an existing resolve error. Skip resolution here.
    if context.error.is_some() {
        return 0;
    }

    let result = std::panic::catch_unwind(AssertUnwindSafe(|| {
        let (_, head) = buffer
            .split_last_mut()
            .expect("XED provided a zero-length symbol buffer");

        let mut writebuf = WriteBuf { data: head };
        match context.cb.resolve(address, &mut writebuf) {
            Ok(Some(symoff)) => *offset = symoff,
            Ok(None) => return false,
            Err(e) => {
                context.error = Some(e);
                return false;
            }
        };

        let end = writebuf.data.as_ptr();
        let range = buffer.as_ptr_range();

        // The only way this can happen is that WriteBuf::write_str panicked but the
        // resolve callback caught and ignored that panic (or we have a bug).
        //
        // Either way something is broken a panicking is the right way to indicate this.
        assert!(range.contains(&end), "corrupted WriteBuf");

        // SAFETY: We just validated that end was within the valid range of the
        //         allocation.
        let index = unsafe { end.offset_from(buffer.as_ptr()) as usize };

        // Ensure that the string we've produced is null-terminated. The resolve
        // function may have written other null bytes to the WriteBuf but that would
        // just result in a shorter string, not any other issues.
        buffer[index] = MaybeUninit::new(0);

        true
    }));

    match result {
        Ok(success) => success.into(),
        Err(payload) => {
            context.panic = Some(payload);
            0
        }
    }
}

impl<'d> DecodedInst<'d> {
    /// Disassemble this instruction using the requested `syntax`.
    ///
    /// This will not resolve any symbols that may be potentially referenced by
    /// the assembly instruction (e.g. function names, block labels, variables,
    /// etc.). To make that work, you will need to call [`disassemble_with`] and
    /// provide a [`DisassemblyCallback`] that can resolve those symbols.
    ///
    /// See the docs on the [`disassemble`](self) module for examples.
    ///
    /// [`disassemble_with`]: DecodedInst::disassemble_with
    pub fn disassemble(&self, syntax: Syntax) -> String {
        self.disassemble_with(&syntax.into())
    }

    /// Disassemble the instruction.
    ///
    /// The provided `options` determine how the disassembly is formatted: what
    /// syntax it should use, symbol rezolution, and more. See
    /// [`DisassembleOptions`] for details on what options are available.
    ///
    /// If your disassembly callback can return an error then you will need to
    /// use [`try_disassemble_with`] instead.
    ///
    /// See the docs on the [`disassemble`](self) module for examples.
    ///
    /// # Panics
    /// - Panics if any of the calls to [`DisassemblyCallback::resolve`] panic.
    /// - Panics if the initial capacity in `options` is greater than
    ///   `i32::MAX`.
    ///
    /// [`try_disassemble_with`]: DecodedInst::try_disassemble_with
    pub fn disassemble_with<F>(&self, options: &DisassembleOptions<F>) -> String
    where
        F: DisassemblyCallback<Error = Infallible>,
    {
        match self.try_disassemble_with(options) {
            Ok(text) => text,
            Err(e) => match e {},
        }
    }

    /// Disassemble the instruction.
    ///
    /// The provided `options` determine how the disassembly is formatted: what
    /// syntax it should use, symbol rezolution, and more. See
    /// [`DisassembleOptions`] for details on what options are available.
    ///
    /// See the docs on the [`disassemble`](self) module for examples.
    ///
    /// # Errors
    /// This function only returns an error if the resolve callback returns an
    /// error. If the resolve callback is infallible (that is, it's error type
    /// is [`Infallible`]) then you can use [`disassemble_with`] to just get
    /// resolved string.
    ///
    /// # Panics
    /// - Panics if any of the calls to [`DisassemblyCallback::resolve`] panic.
    /// - Panics if the initial capacity in `options` is greater than
    ///   `i32::MAX`.
    ///
    /// [`disassemble_with`]: DecodedInst::disassemble_with
    pub fn try_disassemble_with<F>(
        &self,
        options: &DisassembleOptions<F>,
    ) -> Result<String, F::Error>
    where
        F: DisassemblyCallback,
    {
        let mut context = CallbackContext {
            cb: &options.callback,
            error: None,
            panic: None,
        };

        let mut info = MaybeUninit::uninit();
        unsafe { xed_init_print_info(info.as_mut_ptr()) };

        let mut info = unsafe { info.assume_init() };
        info.syntax = options.syntax.into_raw();
        info.format_options = options.options;
        info.format_options_valid = 1;
        info.runtime_address = options.runtime_address;
        info.p = self.as_raw();
        info.context = &mut context as *mut _ as *mut c_void;
        info.disassembly_callback = Some(disassembly_callback::<F>);

        let mut buffer = Vec::<u8>::with_capacity(options.initial_capacity);

        loop {
            let spare = buffer.spare_capacity_mut();
            assert!(
                spare.len() < i32::MAX as usize,
                "disassembly buffer len larger than i32::MAX"
            );

            info.buf = spare.as_mut_ptr() as *mut c_char;
            info.blen = spare.len() as i32;

            let success = unsafe { xed_format_generic(&mut info) };

            if let Some(payload) = context.panic.take() {
                std::panic::resume_unwind(payload);
            }

            if let Some(error) = context.error.take() {
                return Err(error);
            }

            if success != 0 {
                break;
            }

            buffer.reserve(buffer.capacity() * 2);
        }

        // SAFETY: Everything up to the first null byte is guaranteed to be initialized.
        let index = buffer
            .spare_capacity_mut()
            .iter()
            .position(|b| unsafe { b.assume_init_read() == 0 })
            .expect("resulting buffer did not contain a null byte?");

        // SAFETY: We just found the index in spare_capacity_mut so it must be within
        //         the capacity.
        unsafe { buffer.set_len(index) };

        Ok(match String::from_utf8_lossy(&buffer) {
            Cow::Owned(text) => text,
            // SAFETY: from_utf8_lossy just verified that the string is valid utf8
            Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(buffer) },
        })
    }
}

/// Formatting options controlling how XED disassembles instructions.
///
/// See the [module docs](self) for some examples on how to set these.
#[derive(Copy, Clone, Debug)]
pub struct DisassembleOptions<F = ResolveNothing> {
    callback: F,
    options: xed_format_options_t,
    syntax: Syntax,
    runtime_address: u64,
    initial_capacity: usize,
}

impl DisassembleOptions<ResolveNothing> {
    /// Create new options with the default values.
    ///
    /// This uses XED's defaults, which means it is identical to the [`intel`]
    /// function.
    ///
    /// [`intel`]: DisassembleOptions::intel
    pub fn new() -> Self {
        Self::default()
    }

    /// Default options with intel syntax.
    pub fn intel() -> Self {
        Syntax::INTEL.into()
    }

    /// Default options with AT&T syntax.
    pub fn att() -> Self {
        Syntax::ATT.into()
    }
}

impl<F: Default> Default for DisassembleOptions<F> {
    fn default() -> Self {
        Self {
            callback: F::default(),
            options: unsafe { std::mem::zeroed() },
            syntax: Syntax::INTEL,
            runtime_address: 0,
            initial_capacity: 32,
        }
    }
}

impl<F> DisassembleOptions<F> {
    /// Create a new set of options directly from the disassembly callback.
    ///
    /// You will likely want to also set [`runtime_address`] as well. Otherwise,
    /// the addresses passed to the disassembly callback are likely to be
    /// invalid.
    ///
    /// [`runtime_address`]: DisassembleOptions::runtime_address
    pub fn from_callback(callback: F) -> Self
    where
        F: DisassemblyCallback,
    {
        DisassembleOptions::new().callback(callback)
    }

    /// Set the disassembly callback used to resolve addresses to symbols.
    ///
    /// If you set this then you likely want to set [`runtime_address`] as well.
    /// Otherwise, the addresses passed to the disassembly callback are likely
    /// to be invalid.
    ///
    /// [`runtime_address`]: DisassembleOptions::runtime_address
    pub fn callback<Cb>(self, callback: Cb) -> DisassembleOptions<Cb>
    where
        Cb: DisassemblyCallback,
    {
        DisassembleOptions {
            callback,
            options: self.options,
            syntax: self.syntax,
            runtime_address: self.runtime_address,
            initial_capacity: self.initial_capacity,
        }
    }

    /// The instruction syntax to disassemble to.
    ///
    /// XED supports three different instruction syntaxes:
    /// - Intel (via [`Syntax::INTEL`]),
    /// - AT&T (via [`Syntax::ATT`]),
    /// - and a custom XED syntax (via [`Syntax::XED`]).
    ///
    /// Intel and AT&T are usually what users expect to see.
    pub fn syntax(mut self, syntax: Syntax) -> Self {
        self.syntax = syntax;
        self
    }

    /// The address of the instruction in memory.
    ///
    /// This is used in conjunction with a [disassembly callback][cb] to resolve
    /// addresses within the instruction to a symbol and offset combination.
    ///
    /// The default value is 0.
    ///
    /// [cb]: DisassembleOptions::callback
    pub fn runtime_address(mut self, address: u64) -> Self {
        self.runtime_address = address;
        self
    }

    /// The initial capacity to use when creating the output string.
    ///
    /// This will be resized until there is enough room for disassembly to
    /// succeed. Values below 25 will be rounded up to 25.
    pub fn initial_capacity(mut self, capacity: usize) -> Self {
        self.initial_capacity = capacity.min(25);
        self
    }

    /// Whether XED should print the xed address before any symbolic name for
    /// branch targets.
    ///
    /// By default this is false.
    pub fn hex_address_before_symbolic_name(mut self, enabled: bool) -> Self {
        self.options.hex_address_before_symbolic_name = enabled.into();
        self
    }

    /// Print the output in XML format when disassembling to Intel syntax.
    ///
    /// By default this is false.
    pub fn xml_output(mut self, enabled: bool) -> Self {
        self.options.xml_a = enabled.into();
        self
    }

    /// Include flags in the XML formatting.
    ///
    /// This does nothing unless [`xml_output`] is also set to true.
    ///
    /// By default this is false.
    ///
    /// [`xml_output`]: DisassembleOptions::xml_output
    pub fn xml_include_flags(mut self, enabled: bool) -> Self {
        self.options.xml_f = enabled.into();
        self
    }

    /// Omit the unit scale `*1` for displacements.
    ///
    /// By default this is false.
    pub fn omit_unit_scale(mut self, enabled: bool) -> Self {
        self.options.omit_unit_scale = enabled.into();
        self
    }

    /// Whether signed immediates are sign extended.
    ///
    /// By default this is true.
    pub fn sign_extend_signed_immediates(mut self, enabled: bool) -> Self {
        self.options.no_sign_extend_signed_immediates = (!enabled).into();
        self
    }

    /// When writing out a write mask, omit k0.
    ///
    /// By default this is false.
    pub fn write_mask_curly_k0(mut self, enabled: bool) -> Self {
        self.options.write_mask_curly_k0 = enabled.into();
        self
    }

    /// Emit hexadecimal numbers in lowercase.
    ///
    /// By default this is false.
    pub fn lowercase_hex(mut self, enabled: bool) -> Self {
        self.options.lowercase_hex = enabled.into();
        self
    }

    /// Show negative memory displacements as positive numbers.
    pub fn positive_memory_displacement(mut self, enabled: bool) -> Self {
        self.options.positive_memory_displacements = enabled.into();
        self
    }
}

impl From<Syntax> for DisassembleOptions {
    fn from(syntax: Syntax) -> Self {
        DisassembleOptions::new().syntax(syntax)
    }
}

/// A callback to resolve an address to a symbol.
///
/// The address will be determined by XED using the instruction along with the
/// [`runtime_address`] provided with the disassembly options.
///
/// [`runtime_address`]: DisassembleOptions::runtime_address
pub trait DisassemblyCallback {
    type Error;

    /// Given an address, resolve it to a symbol and offset.
    ///
    /// Write the symbol to the provided [`WriteBuf`] and return the offset.
    /// If there is no symbol for the provided `address` then return `Ok(None)`.
    ///
    /// Any returned error will be returned verbatim from
    /// [`DecodedInst::disassemble_with`].
    fn resolve(&self, address: u64, symbol: &mut WriteBuf<'_>) -> Result<Option<u64>, Self::Error>;
}

impl<F> DisassemblyCallback for F
where
    F: Fn(u64, &mut WriteBuf<'_>) -> Option<u64>,
{
    type Error = Infallible;

    fn resolve(&self, address: u64, symbol: &mut WriteBuf<'_>) -> Result<Option<u64>, Self::Error> {
        Ok(self(address, symbol))
    }
}

/// A [`DisassemblyCallback`] that never resolves a symbol.
///
/// This is the default [`DisassemblyCallback`] when constructing a
/// [`DisassembleOptions`].
#[derive(Copy, Clone, Debug, Default)]
pub struct ResolveNothing;

impl DisassemblyCallback for ResolveNothing {
    type Error = std::convert::Infallible;

    fn resolve(&self, _: u64, _: &mut WriteBuf<'_>) -> Result<Option<u64>, Self::Error> {
        Ok(None)
    }
}

/// A writable buffer for storing a symbol name when disassembling.
///
/// XED always provides at most 512 bytes for the disassembled symbol name so
/// the [`fmt::Write`] impl for `WriteBuf` does not return an error when there
/// is no space left and instead just truncates the input.
pub struct WriteBuf<'a> {
    data: &'a mut [MaybeUninit<u8>],
}

impl<'a> WriteBuf<'a> {
    /// Returns the remaining space that is left in the `WriteBuf`.
    pub fn remaining(&self) -> usize {
        self.data.len()
    }

    /// Write new string data to the buffer.
    ///
    /// If there is not enough room left to write `text` then it will be
    /// truncated to fit.
    ///
    /// Note that XED always provides a fixed-size buffer to the disassembly
    /// callback, so if the symbol doesn't fit on the first try then it never
    /// will.
    pub fn extend(&mut self, text: &str) {
        let truncidx = str_floor_char_boundary(text, self.data.len());
        let truncated = &text[..truncidx];

        let (head, rest) = std::mem::take(&mut self.data).split_at_mut(truncidx);
        self.data = rest;

        // SAFETY: truncated is already initialized so transmuting it to MaybeUninit<u8>
        //         is perfectly safe. We also never have to worry about alignment issues
        //         since MaybeUninit<u8> is the same size as u8.
        head.copy_from_slice(unsafe { truncated.as_bytes().align_to().1 });
    }
}

impl<'a> fmt::Write for WriteBuf<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.extend(s);
        Ok(())
    }
}

/// This method is lifted from the unstable `str::floor_char_boundary`.
fn str_floor_char_boundary(s: &str, index: usize) -> usize {
    if index >= s.len() {
        return s.len();
    }

    let lower_bound = index.saturating_sub(3);
    let new_index = s.as_bytes()[lower_bound..=index]
        .iter()
        .rposition(|&b| !(128..192).contains(&b));

    lower_bound + new_index.unwrap()
}
