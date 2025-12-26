/// Core SHA-256 compression and hashing functions.
///
/// Ce module contient la logique principale de SHA-256, dont la fonction de compression de bloc
/// et la fonction haut-niveau [`sha256`] qui traite des entrées de longueur arbitraire.
///
/// # Algorithme
///
/// SHA-256 traite les messages en blocs de 512 bits (64 octets) :
/// 1. Initialisation avec [`H256_INIT`]
/// 2. Padding et encodage de la longueur
/// 3. Compression de chaque bloc
/// 4. Sortie du hash 256 bits
///
/// # Performance
///
/// - Lecture non alignée des blocs
/// - Buffer circulaire 16 mots pour le message schedule (`W`)
///
/// # Fonctions
///
/// - [`compress`] : compresse un bloc 512 bits
/// - [`sha256`] : hash une entrée de taille arbitraire
use crate::hash::sha256::H256_INIT;
use crate::hash::sha256::computations::all_rounds;

use crate::primitives::U256;

/// Compresses a single 512-bit (64-byte) block using the SHA-256 compression function.
///
/// This function implements the main SHA-256 compression function which updates the state
/// based on the input block. The compression consists of 64 rounds of operations.
///
/// # Arguments
///
/// * `block` - A 64-byte block to process
/// * `state` - The current hash state (8 x 32-bit values), updated in-place
///
/// # Details
///
/// The function:
/// 1. Converts the input block into 16 words (32-bit values) in big-endian format
/// 2. Calls the round computation function with the state and word schedule
/// 3. Updates the hash state with the computed values
#[inline(always)]
pub fn compress(block: &[u8; 64], state: &mut [u32; 8]) {
    let mut w = [0u32; 16];

    for (i, slot) in w.iter_mut().enumerate().take(16) {
        // Read u32 in big-endian format
        let idx = i * 4;
        *slot = u32::from_be_bytes([block[idx], block[idx + 1], block[idx + 2], block[idx + 3]]);
    }

    all_rounds(state, w);
}

/// Computes the SHA-256 hash of the input data.
///
/// This function is the main entry point for hashing. It processes the input message
/// of arbitrary length and produces a 256-bit hash value.
///
/// # Arguments
///
/// * `input` - The input data to hash (any length, including zero)
///
/// # Returns
///
/// A [`U256`] containing the 256-bit hash output
///
/// # Algorithm Steps
///
/// 1. **Block Processing**: Iterates through complete 64-byte blocks, compressing each
/// 2. **Padding**: Handles the final partial block with proper SHA-256 padding:
///    - Appends a single '1' bit (0x80 byte)
///    - Pads with zeros to align to 56 bytes (mod 64)
///    - If necessary, creates an additional block for padding
/// 3. **Length Encoding**: Appends the original message length in bits (64-bit big-endian)
/// 4. **Final Compression**: Processes the padded final block(s)
/// 5. **Result**: Returns the hash state as a U256 value
///
/// # Example
///
/// ```ignore
/// let hash = sha256(b"hello");
/// ```
pub fn sha256(input: &[u8]) -> U256 {
    let mut state = H256_INIT;

    let mut i = 0;
    let len = input.len();

    while i + 64 <= len {
        // Convert slice to 64-byte block
        let block: &[u8; 64] = input[i..i + 64].try_into().unwrap();
        compress(block, &mut state);
        i += 64;
    }

    let mut block = [0u8; 64];
    let rem = len - i;

    // Copy remaining bytes and add padding bit
    block[..rem].copy_from_slice(&input[i..]);
    block[rem] = 0x80; // SHA-256 padding bit

    if rem > 55 {
        // Need extra block for message length
        compress(&block, &mut state);
        block = [0; 64];
    }

    let bit_len = (len as u64) << 3; // Convert bytes to bits
    let len_bytes = bit_len.to_be_bytes();

    // Insert message length in the last 8 bytes
    block[56..64].copy_from_slice(&len_bytes);

    compress(&block, &mut state);

    U256::from(state)
}
