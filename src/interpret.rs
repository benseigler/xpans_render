//! Traits concerning source interpreters

/// A source interpreter must implement this.
pub trait InterpretationLength {
    /// The count of `Interpretation` per result slice.
    /// This value applies to each and every interpretation slice.
    /// A renderer should guarantee that the length of each interpretation
    /// slice matches this value.
    fn interpretation_length(&self) -> usize;
}

/// Evaluates an audio source's properties into values used by a sample processor.
pub trait InterpretSource<Source>
where
    Self: InterpretationLength,
{
    /// Source interpretations should be independent of user personalization.
    /// Consideration of user preferences other than things like speaker positions
    /// should be left to the sample processing stage.
    type Interpretation;
    /// Evaluates a single audio source's spatial properties into an
    /// `Interpretation`.
    fn interpret_source(&mut self, source: &Source, result: &mut [Self::Interpretation]);
}
