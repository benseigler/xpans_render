//! Traits concerning sample processors

use crate::interpret::mode::{InterpretationInfo, Interpretations};

/// Uses source interpretations to transform audio samples.
pub trait ProcessSamples<I, In, Out>
where
    I: Interpretations<Self::Interpretation>,
    Self: super::OutputChannels + super::DelaySamples + InterpretationInfo,
{
    /// `In` is likely an implementor of `Input`.
    fn process_samples(&mut self, interpretations: &I, input: &In, output: &mut Out);
}
