//! Traits concerning audio input

/// Contains audio samples from an audio source.
pub trait Input<S> {
    /// Returns the current audio sample for the audio source.
    fn current_sample(&self) -> S;
}

/// An input that specifies its own sample rate.
pub trait SampleRate {
    /// Returns the sample rate for the audio input.
    fn sample_rate(&self) -> u32;
}

/// An input that allows previous samples to be retrieved.
pub trait BufferedInput<S>: Input<S> {
    /// Returns a discrete audio sample within the delay buffer.
    fn integer_sample(&self, index: usize) -> S;
}

/**
An input that allows previous samples to be retrieved with a
fractional index.

`Frac` is usually a floating point type (i.e. f32 or f64).
*/
pub trait FractionalInput<S, Frac>: BufferedInput<S> {
    /**
    Returns an audio sample using a fractional index, which may involve
    resampling, interpolation, etc.
    */
    fn fractional_sample(&self, index: Frac) -> S;
}
