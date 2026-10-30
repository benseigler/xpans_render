//! Traits concerning audio input

/// Contains audio samples for the available audio sources and frames
pub trait Input<S> {
    /// The number of frames available for accessing
    fn frames(&self) -> usize;
    /// The number of audio sources available for accessing
    fn sources(&self) -> usize;
    fn sample(&self, frame: usize, source: usize) -> S;
}

/// An input that specifies its own sample rate.
pub trait SampleRate {
    /// Returns the sample rate for the audio input.
    fn sample_rate(&self) -> u32;
}

/// An input that allows previous samples to be retrieved.
pub trait BackwardIndex<S>: Input<S> {
    /**
    Returns a discrete audio sample within the delay buffer.
    A negative number gets a previous sample, a positive number gets
    a sample ahead of the current frame
    (not exceeding `self.frames() - frame - 1`.)

    Passing `0` into `index` will return the frame's current sample.
    */
    fn relative_sample(&self, frame: usize, source: usize, index: isize) -> S;
    /**
    Returns a discrete audio sample within the delay buffer.

    Passing `0` into `frame` will return the current sample.

    */
    fn previous_sample(&self, frame: usize, source: usize, index: usize) -> S {
        self.relative_sample(frame, source, -index.cast_signed())
    }
    /// The number of samples available behind the current frame, including the current
    /// frame.
    fn previous_frames(&self) -> usize;
}

/**
An input that allows previous samples to be retrieved with a
fractional index.

`Frac` is usually a floating point type (i.e. f32 or f64).
*/
pub trait FractionalIndex<S, Frac>: BackwardIndex<S> {
    /**
    Returns an audio sample using a fractional index, which may involve
    resampling, interpolation, etc.
    */
    fn fractional_sample(&self, frame: usize, source: usize, index: Frac) -> S;
}
