//! ARCompact architecture functionality.

use super::VECTOR_TABLE_SIZE;

/// Check data is an ARCompact vector table.
pub fn is_vector_table(data: &[u8; VECTOR_TABLE_SIZE]) -> bool {
    const SLOT_SIZE: usize = 8;
    const SLOT_PREFIX: &[u8] = &[0x20, 0x20, 0x80, 0x0F]; // j <x>
    const SLOT_COUNT: usize = 3; // Number of slots to check

    data.as_chunks::<SLOT_SIZE>()
        .0
        .iter()
        .take(SLOT_COUNT)
        .all(|x| x.starts_with(SLOT_PREFIX))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn is_vector_table() {
        const DATA_VALID: &[&[u8; VECTOR_TABLE_SIZE]] = &[
            test_data::corsair_nova2::VECTOR_TABLE,
            test_data::phison_s5::VECTOR_TABLE,
            test_data::kingston_ssdnow100::VECTOR_TABLE,
            test_data::apacer_sfd25hm::VECTOR_TABLE,
            test_data::patriot_blaze::VECTOR_TABLE,
            test_data::phison_s9::VECTOR_TABLE,
        ];
        const DATA_INVALID: &[&[u8; VECTOR_TABLE_SIZE]] = &[
            &[0; _],
            &[0xFF; _],
            test_data::westerndigital_scorpioblack::EXCEPTION_VECTOR_TABLE, // ARM32
            test_data::kingston_a400::EXCEPTION_VECTOR_TABLE,               // Xtensa
        ];

        for &data in DATA_VALID {
            assert!(super::is_vector_table(data));
        }

        for &data in DATA_INVALID {
            assert!(!super::is_vector_table(data));
        }
    }
}
