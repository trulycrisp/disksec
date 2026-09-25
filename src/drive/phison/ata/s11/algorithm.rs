//! S11 algorithm implementation.

/// CRC-16 advance variant for cipher.
fn cipher_crc16_advance(mut state: u16, input: u16) -> u16 {
    const POLYNOMIAL: u16 = 0x8005;

    for i in 0..u16::BITS {
        let feedback = (state >> 15) ^ ((input >> i) & 1);
        state <<= 1;

        if feedback != 0 {
            state ^= POLYNOMIAL;
        }
    }

    state
}

/// Generate keystream for CRC-16 based cipher.
fn cipher_crc16_keystream(size: usize, seed: u32, offset: usize) -> Box<[u8]> {
    const BLOCK_SIZE: usize = 32;
    const BLOCKS_PER_CHUNK: usize = 16;
    const CHUNK_SIZE: usize = BLOCK_SIZE * BLOCKS_PER_CHUNK;
    const INITIAL_STATE: u16 = 0x1234;

    let chunk_start = offset / CHUNK_SIZE;
    let chunk_end = (offset + size).div_ceil(CHUNK_SIZE);
    let wanted = offset..offset + size;

    let mut output = Vec::with_capacity(size);
    let mut position = chunk_start * CHUNK_SIZE;

    for chunk in chunk_start..chunk_end {
        let chunk_seed = seed.wrapping_add(chunk.try_into().unwrap());
        let mut state =
            cipher_crc16_advance(INITIAL_STATE, u16::try_from(chunk_seed & 0xFFFF).unwrap());
        state = cipher_crc16_advance(state, (chunk_seed >> 16) as _);

        for _ in 0..BLOCKS_PER_CHUNK {
            let state_reversed = state.reverse_bits();
            let state_not = !state;
            let state_not_reversed = !state_reversed;

            let pattern = [
                (state << 4) & 0xFF00 | (state >> 2) & 0xFF,
                (state_not_reversed << 10) & 0xF000 | (state >> 3) & 0xFFF,
                (state_reversed << 4) & 0xFFF0 | (state_not >> 8) & 0xF,
                (state << 3) & 0xFFF0 | (state_reversed >> 2) & 0xF,
                state_reversed,
                (state_not << 8) | (state >> 8),
                (state_reversed << 4) & 0xF000 | state_not & 0xFFF,
                state,
                (state_reversed << 1) & 0xFFF0 | (state >> 11) & 0xF,
                state_not_reversed,
                state.rotate_right(8),
                state_reversed.rotate_right(8),
                state_not,
                state_reversed,
                (state << 2) & 0xFFF0 | (state_reversed >> 1) & 0xF,
                state,
            ];

            for value in pattern {
                for byte in value.to_le_bytes() {
                    if wanted.contains(&position) {
                        output.push(byte);
                    }
                    position += 1;
                }
                state = cipher_crc16_advance(state, value);
            }
        }
    }

    output.into_boxed_slice()
}

/// Encrypt/decrypt CRC-16 based cipher.
pub(super) fn cipher_crc16(data: &[u8], seed: u32, offset: usize) -> Box<[u8]> {
    let mut keystream = cipher_crc16_keystream(data.len(), seed, offset);

    for (k, d) in keystream.iter_mut().zip(data) {
        *k ^= d;
    }

    keystream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_known_results() {
        assert_eq!(cipher_crc16_advance(0x0000, 0x0000), 0x0000);
        assert_eq!(cipher_crc16_advance(0x1234, 0x0000), 0xECBB);
        assert_eq!(cipher_crc16_advance(0x1234, 0xFFFF), 0x6CB6);
        assert_eq!(cipher_crc16_advance(0xFFFF, 0xFFFF), 0x0000);
        assert_eq!(cipher_crc16_advance(0x1234, 0xABCD), 0x444F);
    }

    #[test]
    fn is_symmetric() {
        const DATA: &[u8] = b"The quick brown fox jumps over the lazy dog.";
        const SEED: u32 = 0x1234_5678;

        let ciphertext = cipher_crc16(DATA, SEED, 0);
        let plaintext = cipher_crc16(&ciphertext, SEED, 0);

        assert_eq!(plaintext.as_ref(), DATA);
    }

    #[test]
    fn output_size_matches_input() {
        for size in [0, 1, 31, 32, 33, 511, 512, 513, 2000] {
            let input = vec![0; size].into_boxed_slice();
            let output = cipher_crc16(&input, 1, 0);

            assert_eq!(input.len(), output.len());
        }
    }

    #[test]
    fn seed_affects_keystream() {
        const SIZE: usize = 64;

        let x = cipher_crc16_keystream(SIZE, 1, 0);
        let y = cipher_crc16_keystream(SIZE, 2, 0);

        assert_ne!(x, y);
    }

    #[test]
    fn keystream_known_result_at_origin() {
        const EXPECTED: &[u8] = &[
            0xE5, 0xB9, 0x72, 0xAD, 0x64, 0x9D, 0xB5, 0x5C, 0xD6, 0x69, 0x6B, 0x69, 0x69, 0x94,
            0x96, 0x6B, 0xAD, 0xD3, 0x29, 0x96, 0x6B, 0x96, 0x69, 0xD6, 0x69, 0x94, 0xD6, 0x69,
            0x5B, 0xAE, 0x96, 0x6B,
        ];

        let keystream = cipher_crc16_keystream(EXPECTED.len(), 0, 0);

        assert_eq!(keystream.as_ref(), EXPECTED);
    }

    #[test]
    fn keystream_known_result_across_chunk_boundary() {
        const SEED: u32 = 0xDEAD_BEEF;
        const OFFSET: usize = 504;
        const EXPECTED: &[u8] = &[
            0x98, 0x59, 0x65, 0xE6, 0x92, 0x99, 0x67, 0xA6, 0xA8, 0xEA, 0xD4, 0x13, 0x91, 0x57,
            0x0E, 0xF5,
        ];

        let keystream = cipher_crc16_keystream(EXPECTED.len(), SEED, OFFSET);

        assert_eq!(keystream.as_ref(), EXPECTED);
    }

    #[test]
    fn offset_works() {
        const FULL_SIZE: usize = 128;
        const OFFSET: usize = 48;
        const SEED: u32 = 0x89AB_CDEF;

        let full_data = cipher_crc16_keystream(FULL_SIZE, SEED, 0);
        let offset_data = cipher_crc16_keystream(FULL_SIZE - OFFSET, SEED, OFFSET);

        assert_eq!(&full_data[OFFSET..], offset_data.as_ref());
    }
}
