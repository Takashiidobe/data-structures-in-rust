use std::hash::Hasher;

pub struct Fnv32Hasher(u32);
pub struct Fnv64Hasher(u64);

impl Fnv32Hasher {
    pub fn seed(seed: u32) -> Fnv32Hasher {
        Fnv32Hasher(seed)
    }
}

impl Default for Fnv32Hasher {
    fn default() -> Fnv32Hasher {
        Fnv32Hasher(2166136261)
    }
}

impl Hasher for Fnv32Hasher {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.0 = self.0 ^ (*byte as u32);
            self.0 = self.0.wrapping_mul(16777619);
        }
    }
    fn finish(&self) -> u64 {
        self.0.into()
    }
}

impl Fnv64Hasher {
    pub fn seed(seed: u64) -> Fnv64Hasher {
        Fnv64Hasher(seed)
    }
}

impl Default for Fnv64Hasher {
    fn default() -> Fnv64Hasher {
        Fnv64Hasher(14695981039346656037)
    }
}

impl Hasher for Fnv64Hasher {
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes.iter() {
            self.0 = self.0 ^ (*byte as u64);
            self.0 = self.0.wrapping_mul(1099511628211);
        }
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn fnv_64(bytes: &[u8]) -> u64 {
        let mut hasher = Fnv64Hasher::default();
        hasher.write(bytes);
        hasher.finish()
    }

    fn fnv_32(bytes: &[u8]) -> u64 {
        let mut hasher = Fnv32Hasher::default();
        hasher.write(bytes);
        hasher.finish()
    }

    #[test]
    fn empty() {
        assert_eq!(fnv_64(b""), 14695981039346656037);
        assert_eq!(fnv_32(b""), 2166136261);
    }
}
