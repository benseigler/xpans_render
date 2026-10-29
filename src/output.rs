//! Traits concerning audio output

/// Contains rendered output samples for each audio source.
pub trait Output<S> {
    /**
    Sample processors will call this function every channel, for every source,
    for every frame.

    The `channel` argument should be between zero and the sample
    processor's `channel_count`.

    Sumnation is not performed by the sample processor. Implementors of this
    trait may choose to perform sumnation within this method or at a
    later time.
    */
    fn set_channel(&mut self, channel: usize, sample: S);
}
