/// A pure-Rust implementation of ChaCha8 for Monero compatibility
/// Matches Monero's original C `chacha8` layout: 64-bit nonce, 8 rounds

use std::convert::TryInto;

const SIGMA: [u32; 4] = [
    0x61707865, // "expa"
    0x3320646e, // "nd 3"
    0x79622d32, // "2-by"
    0x6b206574, // "te k"
];

#[inline(always)]
fn rotl(a: u32, b: u32) -> u32 {
    a.rotate_left(b)
}

#[inline(always)]
fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
    state[a] = state[a].wrapping_add(state[b]); state[d] ^= state[a]; state[d] = rotl(state[d], 16);
    state[c] = state[c].wrapping_add(state[d]); state[b] ^= state[c]; state[b] = rotl(state[b], 12);
    state[a] = state[a].wrapping_add(state[b]); state[d] ^= state[a]; state[d] = rotl(state[d], 8);
    state[c] = state[c].wrapping_add(state[d]); state[b] ^= state[c]; state[b] = rotl(state[b], 7);
}

pub fn chacha8_monero(key: &[u8; 32], nonce: &[u8; 8], input: &[u8]) -> Vec<u8> {
    let mut output = vec![0u8; input.len()];
    let mut block = [0u8; 64];

    let mut counter: u64 = 0;

    for (chunk_idx, chunk) in input.chunks(64).enumerate() {
        // Setup state
        let mut state = [0u32; 16];

        state[0..4].copy_from_slice(&SIGMA);
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes(key[i * 4..(i + 1) * 4].try_into().unwrap());
        }
        state[12] = (counter & 0xffffffff) as u32;
        state[13] = (counter >> 32) as u32;
        state[14] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        state[15] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());

        let initial = state;

        // 8 rounds (4 double-rounds)
        for _ in 0..4 {
            // Column rounds
            quarter_round(&mut state, 0, 4, 8, 12);
            quarter_round(&mut state, 1, 5, 9, 13);
            quarter_round(&mut state, 2, 6, 10, 14);
            quarter_round(&mut state, 3, 7, 11, 15);
            // Diagonal rounds
            quarter_round(&mut state, 0, 5, 10, 15);
            quarter_round(&mut state, 1, 6, 11, 12);
            quarter_round(&mut state, 2, 7, 8, 13);
            quarter_round(&mut state, 3, 4, 9, 14);
        }

        for i in 0..16 {
            let word = state[i].wrapping_add(initial[i]).to_le_bytes();
            block[i * 4..(i + 1) * 4].copy_from_slice(&word);
        }

        let block_len = chunk.len();
        for i in 0..block_len {
            output[chunk_idx * 64 + i] = chunk[i] ^ block[i];
        }

        counter += 1;
    }

    output
}

// Example usage:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha8_monero_basic() {
        let key = [0u8; 32];
        let nonce = [0u8; 8];
        let plaintext = [0u8; 64];
        let ciphertext = chacha8_monero(&key, &nonce, &plaintext);
        assert_eq!(ciphertext.len(), 64);
    }
}
