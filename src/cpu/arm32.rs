//! ARM32 architecture functionality.

use super::VECTOR_TABLE_SIZE;

/// ARM 32-bit instruction size.
pub const INSTRUCTION_SIZE: usize = 4;

/// Check data is an ARM exception vector table.
pub fn is_exception_vector_table(data: &[u8; VECTOR_TABLE_SIZE]) -> bool {
    const LDR_PC: &[u8; INSTRUCTION_SIZE] = &[0x0, 0xF0, 0x9F, 0xE5]; // ldr pc, [pc, #<x>]
    const LDR_PC_MASK: &[u8; INSTRUCTION_SIZE] = &[0x0, 0xF0, 0xFF, 0xFF];
    const INSTRUCTION_COUNT: usize = 3; // Number of instructions to check

    data.as_chunks::<INSTRUCTION_SIZE>()
        .0
        .iter()
        .take(INSTRUCTION_COUNT)
        .all(|instruction| {
            instruction
                .iter()
                .zip(LDR_PC)
                .zip(LDR_PC_MASK)
                .all(|((x, y), m)| x & m == y & m)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn is_exception_vector_table() {
        const DATA_VALID: &[&[u8; VECTOR_TABLE_SIZE]] = &[
            test_data::westerndigital_scorpioblack::EXCEPTION_VECTOR_TABLE,
            test_data::westerndigital_bluemobile::EXCEPTION_VECTOR_TABLE,
            test_data::seagate_momentus5::EXCEPTION_VECTOR_TABLE,
            test_data::kingston_dc500r::EXCEPTION_VECTOR_TABLE,
        ];
        const DATA_INVALID: &[&[u8; VECTOR_TABLE_SIZE]] = &[
            &[0; _],
            &[0xFF; _],
            test_data::phison_s5::VECTOR_TABLE, // ARCompact
            test_data::kingston_a400::EXCEPTION_VECTOR_TABLE, // Xtensa
        ];

        for &data in DATA_VALID {
            assert!(super::is_exception_vector_table(data));
        }

        for &data in DATA_INVALID {
            assert!(!super::is_exception_vector_table(data));
        }
    }
}
