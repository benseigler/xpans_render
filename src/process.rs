pub mod host;
pub mod mode;

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
