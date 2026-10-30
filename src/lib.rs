/*!
An interface for defining spatial audio rendering modes and
implementing mode-agnostic renderers for the xpans Ecosystem

## Examples
For examples on how to implement rendering modes, refer to
[xpans Stereo](https://github.com/xpans-audio/xpans_stereo) and
[xpans Headphones](https://github.com/xpans-audio/xpans_headphones).

For an example on how to implement a host, refer to
[xpans Violet](https://github.com/xpans-audio/xpans_violet).
*/
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
mod std;

mod input;
mod interpret;
mod output;
mod process;

/// Imports for hosts
pub mod host {
    pub use crate::input::*;
    pub use crate::output::*;

    pub use crate::interpret::host::{InterpretSources, InterpretationInfo, Interpretations};
    pub use crate::process::host::ProcessSamples;

    pub use crate::interpret::Sources;
    pub use crate::process::{AcceptableFrames, DelaySamples, MinimumFrames, OutputChannels};
}

/// Imports for modes
pub mod mode {
    pub use crate::input::*;
    pub use crate::output::*;

    pub use crate::interpret::mode::{InterpretSources, InterpretationInfo, Interpretations};
    pub use crate::process::mode::ProcessSamples;

    pub use crate::interpret::Sources;
    pub use crate::process::{AcceptableFrames, DelaySamples, MinimumFrames, OutputChannels};
}
