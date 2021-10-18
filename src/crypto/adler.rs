use std::hash::Hasher;

/// A hasher that returns an adler32 sum.
pub struct Adler32Hasher(u32, u32, u32);

impl Adler32Hasher {
    /// The seed provided to the Hasher.
    pub fn seed(seed: u32) -> Adler32Hasher {
        Adler32Hasher(seed, 1, 0)
    }
}

impl Default for Adler32Hasher {
    fn default() -> Adler32Hasher {
        Adler32Hasher(65521, 1, 0)
    }
}

impl Hasher for Adler32Hasher {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.1 = (self.1 + *byte as u32) % self.0;
            self.2 = (self.2 + self.1) % self.0;
        }
    }

    fn finish(&self) -> u64 {
        (self.2 << 16 | self.1).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn adler_32(bytes: &[u8]) -> u64 {
        let mut hasher = Adler32Hasher::default();
        hasher.write(bytes);
        hasher.finish()
    }

    #[test]
    fn empty() {
        assert_eq!(adler_32(b""), 1);
    }

    #[test]
    fn test_123() {
        assert_eq!(adler_32(b"123"), 19726487);
    }
}
