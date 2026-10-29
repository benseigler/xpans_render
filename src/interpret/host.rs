use core::alloc::Layout;

use crate::mode;

/**
Low-level memory information concerning interpretation types

This trait is specifically for allowing easier type erasure of interpretation types.

# Safety
Incorrect implementations of this trait will cause undefined behavior
in code that relies on this trait's methods. Implementors must ensure
that `interpretation_layout()` reports the type's layout accurately.

Implementing `initialize_interpretation()` demands low-level
understanding of the interpretation type and will almost certainly
require writing unsafe code. Calling the method should result in `bytes`
representing a valid and safe instance of the interpretation type.
*/
pub unsafe trait InterpretationInfo {
    /// Returns the memory layout of the interpretation type
    fn interpretation_layout(&self) -> Layout;
    /// Returns the number of interpretation values per source, per frame
    fn interpretation_length(&self) -> usize;
    /**
    Initializes a single interpretation value into a slice of bytes

    # Safety
    The length of `bytes` must equal `self.interpretation_layout().size()`.
    */
    unsafe fn initialize_interpretation(&self, bytes: &mut [u8]);
}

unsafe impl<T> InterpretationInfo for T
where
    T: crate::interpret::mode::InterpretationInfo,
{
    fn interpretation_layout(&self) -> Layout {
        Layout::new::<T::Interpretation>()
    }

    fn interpretation_length(&self) -> usize {
        self.interpretation_length()
    }
    unsafe fn initialize_interpretation(&self, bytes: &mut [u8]) {
        let init = T::Interpretation::default();
        let ptr = &init as *const T::Interpretation;
        let erased = ptr as *const u8;
        let size = self.interpretation_layout().size();
        let slice = unsafe { core::slice::from_raw_parts(erased, size) };
        bytes.copy_from_slice(slice);
    }
}

/**
Allows for accessing spatial interpretations

# Safety
This trait is designed to operate on type erased interpretation data.
Implementing this trait will almost certainly involve writing
unsafe code.
*/
pub unsafe trait Interpretations {
    /// Returns the number of interpretation values per source, per frame
    fn length(&self) -> usize;
    /**
    Returns an immutable reference to the interpretation value at `index`.

    # Safety
    `T` must be the same type as the erased type being accessed.
    */
    unsafe fn get<T>(&self, index: usize) -> &T;
    /**
    Returns a mutable reference to the interpretation value at `index`.

    # Safety
    `T` must be the same type as the erased type being accessed.
    */
    unsafe fn get_mut<T>(&mut self, index: usize) -> &mut T;
    /**
    Sets the interpretation value at `index`.

    # Safety
    `T` must be the same type as the erased type being accessed.
    */
    unsafe fn set<T>(&mut self, index: usize, value: T) {
        unsafe { *self.get_mut(index) = value };
    }
}

impl<I: Interpretations, T> mode::Interpretations<T> for I {
    fn length(&self) -> usize {
        self.length()
    }

    fn get(&self, index: usize) -> &T {
        unsafe { self.get::<T>(index) }
    }

    fn get_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.get_mut::<T>(index) }
    }

    fn set(&mut self, index: usize, value: T) {
        unsafe { self.set::<T>(index, value) }
    }
}

/**
Evaluates an audio source's properties into values used by a sample processor.

# Safety
Implementors must ensure memory and type safety when accessing interpretations.
*/
pub unsafe trait InterpretSource<Source, I>
where
    I: Interpretations,
    Self: InterpretationInfo,
{
    /**
    Evaluates a single audio source's spatial properties into a
    spatial interpretation.
    */
    fn interpret_source(&mut self, source: &Source, interpretations: &mut I);
}

unsafe impl<Source, I, T> InterpretSource<Source, I> for T
where
    I: Interpretations + mode::Interpretations<T::Interpretation>,
    T: crate::interpret::mode::InterpretSource<Source, I>,
{
    fn interpret_source(&mut self, source: &Source, interpretations: &mut I) {
        self.interpret_source(source, interpretations);
    }
}
