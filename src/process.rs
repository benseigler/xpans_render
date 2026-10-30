pub mod host;
pub mod mode;

/// A sample processor that specifies the number of output channels it renders to.
pub trait OutputChannels {
    /// Returns the number of output channels the rendering mode will render to.
    fn output_channels(&self) -> usize;
}

/**
A sample processor that specifies the maximum number of buffered samples
required by this rendering mode.
*/
pub trait DelaySamples {
    /// Returns the maximum number of buffered samples required by this rendering mode.
    fn delay_samples(&self, sample_rate: u32) -> usize;
}

/// Tell a host how many frames a processor can accept
pub trait AcceptableFrames {
    /**
    The renderer/host will call this method on the processor to ask how
    many of the renderer's available frames the processor can render at the time.

    This is useful if the processor can only render a multiple of a
    certain number of frames at a time.

    `frames` is the number of frames the host has available, and the
    return value is the number of frames the processor can accept.
    */
    fn acceptable_frames(&self, sample_rate: u32, frames: usize) -> usize;
}

/// Tell a host how many frames must be available to process audio
pub trait MinimumFrames {
    /**
    The renderer/host will call this method on the processor to ask how
    many frames must be available in order for it to process audio.

    The host should never call the sample processor's `process_samples()`
    method unless the frames it has available is greater than or equal
    to what this method returns.
    */
    fn minimum_frames(&self, sample_rate: u32) -> usize;
}
