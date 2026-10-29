use crate::{
    interpret::{self},
    process::{DelaySamples, OutputChannels},
};

/**
Uses source interpretations to transform audio samples.

# Safety
Implementors must ensure memory and type safety when accessing interpretations.
*/
pub unsafe trait ProcessSamples<I, In, Out>
where
    I: crate::interpret::host::Interpretations,
    Self: OutputChannels + DelaySamples,
{
    /// Input is likely an implementor of `Input`.
    fn process_samples(&mut self, interpretations: &I, input: &In, output: &mut Out);
}

unsafe impl<I, In, Out, T> ProcessSamples<I, In, Out> for T
where
    I: interpret::mode::Interpretations<T::Interpretation>
        + crate::interpret::host::Interpretations,
    T: OutputChannels + DelaySamples + crate::process::mode::ProcessSamples<I, In, Out>,
{
    fn process_samples(&mut self, interpretations: &I, input: &In, output: &mut Out) {
        self.process_samples(interpretations, input, output);
    }
}
