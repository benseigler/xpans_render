//! Traits concerning sample processors

/// Uses source interpretations to transform audio samples.
pub trait ProcessSamples<In, Out>
where
    Self: OutputChannels + DelaySamples,
{
    /// This type must match that of any implementors of `InterpretSource` used
    /// with this processor.
    type Interpretation;
    /// Input is likely an implementor of `Input`.
    fn process_samples(&mut self, result: &[Self::Interpretation], input: &In, output: &mut Out);
}

/// A sample processor that specifies the number of output channels it renders to.
pub trait OutputChannels {
    /// Returns the number of output channels the rendering mode will render to.
    fn output_channels(&self) -> usize;
}

/// A sample processor that specifies the maximum number of buffered samples
/// required by this rendering mode.
pub trait DelaySamples {
    /// Returns the maximum number of buffered samples required by this rendering mode.
    fn delay_samples(&self, sample_rate: u32) -> usize;
}
