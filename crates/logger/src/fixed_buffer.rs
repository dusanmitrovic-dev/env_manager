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
pub struct FixedBuffer<const N: usize> {
    bytes: [u8; N],
    position: usize,
}

impl<const N: usize> FixedBuffer<N> {
    /// Returns a new, empty [FixedBuffer] initialized with zeros.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bytes: [0u8; N],
            position: 0,
        }
    }

    /// Returns the written portion of the buffer as a valid UTF-8 string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        from_utf8(&self.bytes[..self.position]).unwrap_or("")
    }
}

impl<const N: usize> Default for FixedBuffer<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Write for FixedBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let remaining = self.bytes.len().saturating_sub(self.position);

        if remaining == 0 {
            return Ok(());
        }

        let bytes = s.as_bytes();
        let mut to_copy = bytes.len().min(remaining);

        if to_copy < bytes.len() {
            while to_copy > 0 && !s.is_char_boundary(to_copy) {
                to_copy -= 1;
            }
        }

        self.bytes[self.position..self.position + to_copy].copy_from_slice(&bytes[..to_copy]);
        self.position += to_copy;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;

    /// Tests base workflow when writing string to a fixed buffer.
    #[test]
    fn test_buffer_write_str() {
        let mut buffer = FixedBuffer::<7>::new();

        assert!(Write::write_str(&mut buffer, "abc🦀").is_ok());
        assert_eq!(buffer.as_str(), "abc🦀");
    }

    /// Tests if safe truncation happens when writing utf8 string to a fixed
    /// buffer without cutting the emoji due to small buffer size.
    #[test]
    fn test_utf8_truncation_on_buffer_write_str() {
        let mut buffer_overflow = FixedBuffer::<5>::new();

        assert!(Write::write_str(&mut buffer_overflow, "abc🦀").is_ok());
        assert_eq!(buffer_overflow.as_str(), "abc");
    }
}
