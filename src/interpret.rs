pub mod host;
pub mod mode;

/// Contains spatial samples for the available audio sources and frames
pub trait Sources {
    type Source;
    /// The number of audio sources available for accessing
    fn sources(&self) -> usize;
    /// The number of frames available for accessing
    fn frames(&self) -> usize;
    /// Retrieve the specified audio source at the specified frame
    fn get(&self, frame: usize, source: usize) -> Self::Source;
}
