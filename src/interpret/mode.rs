//! Traits concerning source interpreters

/// For specifying and accessing interpretation type and length information.
pub trait InterpretationInfo {
    /// The type of the interpretation values
    type Interpretation: Default;
    /// Returns the number of interpretation values per source, per frame
    fn interpretation_length(&self) -> usize;
}

/// Allows for accessing spatial interpretations
pub trait Interpretations<T> {
    /// The number of frames available for accessing
    fn frames(&self) -> usize;
    /// The number of audio sources available for accessing
    fn sources(&self) -> usize;
    /// Returns the number of interpretation values per source, per frame
    fn length(&self) -> usize;
    /// Returns an immutable reference to the interpretation value at `index`.
    fn get(&self, frame: usize, source: usize, index: usize) -> &T;
    /// Returns a mutable reference to the interpretation value at `index`.
    fn get_mut(&mut self, frame: usize, source: usize, index: usize) -> &mut T;
    /// Sets the interpretation value at `index`.
    fn set(&mut self, frame: usize, source: usize, index: usize, value: T) {
        *self.get_mut(frame, source, index) = value
    }
}

/// Evaluates an audio source's properties into values used by a sample processor.
pub trait InterpretSources<Sources, I>
where
    I: Interpretations<Self::Interpretation>,
    Self: InterpretationInfo,
{
    /**
    Evaluates a single audio source's spatial properties into a
    spatial interpretation.
    */
    fn interpret_sources(&mut self, sources: &Sources, interpretations: &mut I);
}
