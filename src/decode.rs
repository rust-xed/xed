use std::mem::MaybeUninit;

use xed_sys::*;

use super::{ChipFeatures, DecodedInst, Error, State};

/// Options for [`decode`].
///
/// In order to decode instructions you must always specify a [`State`] but you
/// can also specify a set of chip features (via [`ChipFeatures`]) that restrict
/// what instructions are considered to be valid.
pub struct DecodeOptions {
    state: State,
    features: Option<ChipFeatures>,
}

impl DecodeOptions {
    /// Create a new set of options from a [`State`].
    pub fn new(state: State) -> Self {
        Self {
            state,
            features: None,
        }
    }

    /// Restrict the set of valid instructions to only those belonging to the
    /// features specified in [`ChipFeatures`].
    pub fn features(mut self, features: ChipFeatures) -> Self {
        self.features = Some(features);
        self
    }
}

/// Decode an instruction from some bytes.
///
/// This is the main interface to the decoder.
///
/// # Parameters
/// - `itext` - A slice containing the instruction bytes. XED will never access
///   anything beyond the 15th byte.
/// - `options` - A set of [`DecodeOptions`] that control how the instruction is
///   decoded.
pub fn decode(itext: &[u8], options: DecodeOptions) -> Result<DecodedInst<'_>, Error> {
    let mut inst = MaybeUninit::uninit();
    unsafe { xed_decoded_inst_zero_set_mode(inst.as_mut_ptr(), options.state.as_raw()) };

    let result = match options.features {
        Some(mut features) => unsafe {
            xed_decode_with_features(
                inst.as_mut_ptr(),
                itext.as_ptr(),
                itext.len() as u32,
                features.as_raw_mut(),
            )
        },
        None => unsafe { xed_decode(inst.as_mut_ptr(), itext.as_ptr(), itext.len() as u32) },
    };

    match Error::from_raw(result) {
        Some(err) => Err(err),
        None => Ok(unsafe { DecodedInst::from_raw(inst.assume_init()) }),
    }
}
