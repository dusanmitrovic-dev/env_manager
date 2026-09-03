use core::str::from_utf8;
use std::fmt;

/// A compile-time bounded, fixed-capacity byte buffer with no heap string
/// formatting.
///
/// Implements [fmt::Write] to format data directly into pre-allocated byte
/// array without dynamic heap allocation.
///
/// # Fields:
///
/// - [bytes]: Fixed-size array allocated directly within the struct.
/// - [position]: Current write cursor tracking the number of written bytes.
struct FixedBuffer<const N: usize> {
    bytes: [u8; N],
    position: usize,
}

impl<const N: usize> FixedBuffer<N> {
    /// Returns a new, empty [FixedBuffer] initialized with zeros.
    const fn new() -> Self {
        Self {
            bytes: [0u8; N],
            position: 0,
        }
    }

    /// Returns the written portion of the buffer as a valid UTF-8 string slice.
    fn as_str(&self) -> &str {
        from_utf8(&self.bytes[..self.position]).unwrap_or("")
    }
}

impl<const N: usize> fmt::Write for FixedBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let remaining = self.bytes.len().saturating_sub(self.position);
        let to_copy = bytes.len().min(remaining);

        self.bytes[self.position..self.position + to_copy].copy_from_slice(&bytes[..to_copy]);
        self.position += to_copy;

        Ok(())
    }
}
