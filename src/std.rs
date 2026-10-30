use crate::mode;

impl<T> mode::InterpretationInfo for Box<T>
where
    T: ?Sized + mode::InterpretationInfo,
{
    type Interpretation = T::Interpretation;
    fn interpretation_length(&self) -> usize {
        self.as_ref().interpretation_length()
    }
}

impl<Sources, I, T> mode::InterpretSources<Sources, I> for Box<T>
where
    I: mode::Interpretations<T::Interpretation>,
    T: ?Sized + mode::InterpretSources<Sources, I>,
{
    fn interpret_sources(&mut self, sources: &Sources, interpretations: &mut I) {
        self.as_mut().interpret_sources(sources, interpretations);
    }
}

impl<T> crate::process::DelaySamples for Box<T>
where
    T: ?Sized + crate::process::DelaySamples,
{
    fn delay_samples(&self, sample_rate: u32) -> usize {
        self.as_ref().delay_samples(sample_rate)
    }
}

impl<T> crate::process::OutputChannels for Box<T>
where
    T: ?Sized + crate::process::OutputChannels,
{
    fn output_channels(&self) -> usize {
        self.as_ref().output_channels()
    }
}

impl<T> crate::process::MinimumFrames for Box<T>
where
    T: ?Sized + crate::process::MinimumFrames,
{
    fn minimum_frames(&self, sample_rate: u32) -> usize {
        self.as_ref().minimum_frames(sample_rate)
    }
}

impl<T> crate::process::AcceptableFrames for Box<T>
where
    T: ?Sized + crate::process::AcceptableFrames,
{
    fn acceptable_frames(&self, sample_rate: u32, frames: usize) -> usize {
        self.as_ref().acceptable_frames(sample_rate, frames)
    }
}

impl<T, I, In, Out> mode::ProcessSamples<I, In, Out> for Box<T>
where
    I: mode::Interpretations<T::Interpretation>,
    T: ?Sized + mode::ProcessSamples<I, In, Out>,
{
    fn process_samples(&mut self, interpretations: &I, input: &In, output: &mut Out) {
        self.as_mut()
            .process_samples(interpretations, input, output);
    }
}
