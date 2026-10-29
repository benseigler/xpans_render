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
    /// Returns the number of interpretation values per source, per frame
    fn length(&self) -> usize;
    /// Returns an immutable reference to the interpretation value at `index`.
    fn get(&self, index: usize) -> &T;
    /// Returns a mutable reference to the interpretation value at `index`.
    fn get_mut(&mut self, index: usize) -> &mut T;
    /// Sets the interpretation value at `index`.
    fn set(&mut self, index: usize, value: T) {
        *self.get_mut(index) = value
    }
}

/// Evaluates an audio source's properties into values used by a sample processor.
pub trait InterpretSource<Source, I>
where
    I: Interpretations<Self::Interpretation>,
    Self: InterpretationInfo,
{
    /**
    Evaluates a sisngle audio source's spatial properties into a
    spatial interpretation.
    */
    fn interpret_source(&mut self, source: &Source, interpretations: &mut I);
}
