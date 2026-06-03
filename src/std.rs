use crate::{
    interpret::InterpretSource, interpret::InterpretationLength, process::DelaySamples,
    process::OutputChannels, process::ProcessSamples,
};

impl<T> InterpretationLength for Box<T>
where
    T: ?Sized + InterpretationLength,
{
    fn interpretation_length(&self) -> usize {
        self.as_ref().interpretation_length()
    }
}

impl<Source, T> InterpretSource<Source> for Box<T>
where
    T: ?Sized + InterpretSource<Source>,
{
    type Interpretation = T::Interpretation;

    fn interpret_source(&mut self, source: &Source, result: &mut [Self::Interpretation]) {
        self.as_mut().interpret_source(source, result);
    }
}

impl<T> DelaySamples for Box<T>
where
    T: ?Sized + DelaySamples,
{
    fn delay_samples(&self, sample_rate: u32) -> usize {
        self.as_ref().delay_samples(sample_rate)
    }
}

impl<T> OutputChannels for Box<T>
where
    T: ?Sized + OutputChannels,
{
    fn output_channels(&self) -> usize {
        self.as_ref().output_channels()
    }
}

impl<T, In, Out> ProcessSamples<In, Out> for Box<T>
where
    T: ?Sized + ProcessSamples<In, Out>,
{
    type Interpretation = T::Interpretation;

    fn process_samples(&mut self, result: &[Self::Interpretation], input: &In, output: &mut Out) {
        self.as_mut().process_samples(result, input, output);
    }
}
