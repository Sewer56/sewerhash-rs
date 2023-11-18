/// Non-vectorised implementation of the Unstable Hash algorithm.
///
/// This is a variation of the FNV-1a hash algorithm, but with the following characteristics
///
/// - Unrolled FNV-1a 64bit (consuming `u64` bytes at one time, into 2 hashes).
/// - Hashing remaining 2/4/6 bytes by `2 bytes` at a time.
/// - Mixing the 2 hashes.
pub fn unstable_hash_non_vector64(data: &[u8]) -> usize {
    unsafe {
        let length = data.len();
        const PRIME: u64 = 0x00000100000001B3;
        let mut hash1: u64 = 0xcbf29ce484222325;
        let mut hash2 = hash1;

        let mut ptr = data.as_ptr() as *const u64;
        let mut remaining_length = length;

        // 64 byte loop
        while remaining_length >= 64 {
            remaining_length -= 64;
            hash1 = (hash1 ^ ptr.read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(1).read()).wrapping_mul(PRIME);
            hash1 = (hash1 ^ ptr.add(2).read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(3).read()).wrapping_mul(PRIME);
            hash1 = (hash1 ^ ptr.add(4).read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(5).read()).wrapping_mul(PRIME);
            hash1 = (hash1 ^ ptr.add(6).read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(7).read()).wrapping_mul(PRIME);
            ptr = ptr.add(8);
        }

        // 32 byte
        if remaining_length >= 32 {
            remaining_length -= 32;
            hash1 = (hash1 ^ ptr.read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(1).read()).wrapping_mul(PRIME);
            hash1 = (hash1 ^ ptr.add(2).read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(3).read()).wrapping_mul(PRIME);
            ptr = ptr.add(4);
        }

        // 16 byte
        if remaining_length >= 16 {
            remaining_length -= 16;
            hash1 = (hash1 ^ ptr.read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(1).read()).wrapping_mul(PRIME);
            ptr = ptr.add(2);
        }

        // 8 byte
        if remaining_length >= 8 {
            remaining_length -= 8;
            hash1 = (hash1 ^ ptr.read()).wrapping_mul(PRIME);
            hash2 = (hash2 ^ ptr.add(1).read()).wrapping_mul(PRIME);
            ptr = ptr.add(1);
        }

        // 2/4/6 bytes left
        let mut remaining_ptr = ptr as *const u16;
        if remaining_length >= 2 {
            remaining_length -= 2;
            hash1 = (hash1 ^ remaining_ptr.read() as u64).wrapping_mul(PRIME);
            hash2 = (hash2 ^ remaining_ptr.add(1).read() as u64).wrapping_mul(PRIME);
            remaining_ptr = remaining_ptr.add(2);
        }

        if remaining_length >= 1 {
            hash2 = (hash2 ^ remaining_ptr.read() as u64).wrapping_mul(PRIME);
        }

        (hash1.wrapping_add(hash2.wrapping_mul(1566083941))) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let data = b"";
        let hash = unstable_hash_non_vector64(data);
        assert_ne!(hash, 0); // Assuming hash of empty data is non-zero
    }

    #[test]
    fn test_small_data() {
        let data = b"hello";
        let hash = unstable_hash_non_vector64(data);
        assert_ne!(hash, 0); // Hash should be non-zero
    }

    #[test]
    fn test_medium_data() {
        let data = b"The quick brown fox jumps over the lazy dog";
        let hash = unstable_hash_non_vector64(data);
        assert_ne!(hash, 0); // Hash should be non-zero
    }

    #[test]
    fn test_large_data() {
        let data = &[0u8; 1024]; // 1 KB of zeros
        let hash = unstable_hash_non_vector64(data);
        assert_ne!(hash, 0); // Hash should be non-zero
    }

    #[test]
    fn test_consistency() {
        let data = b"consistent data";
        let hash1 = unstable_hash_non_vector64(data);
        let hash2 = unstable_hash_non_vector64(data);
        assert_eq!(hash1, hash2); // Hash should be the same across calls
    }

    #[test]
    fn test_different_data() {
        let data1 = b"data1";
        let data2 = b"data2";
        let hash1 = unstable_hash_non_vector64(data1);
        let hash2 = unstable_hash_non_vector64(data2);
        assert_ne!(hash1, hash2); // Hash should differ for different data
    }
}
