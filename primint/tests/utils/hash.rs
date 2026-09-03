use std::hash::{Hash, Hasher};

pub fn hashed_bytes<T: Hash>(value: T) -> Vec<u8> {
    struct DummyHasher {
        bytes: Vec<u8>,
    }
    impl Hasher for DummyHasher {
        fn finish(&self) -> u64 {
            unimplemented!()
        }
        fn write(&mut self, bytes: &[u8]) {
            self.bytes.extend(bytes);
        }
    }
    let mut hasher = DummyHasher { bytes: Vec::new() };
    value.hash(&mut hasher);
    hasher.bytes
}
