use crate::character::Character;

/// Experience points required to reach each level (index = level)
/// Level 0 requires 0 XP, Level 1 requires 0 XP (starting level),
/// Level 2 requires 300 XP, etc.
pub const XP_TABLE: [u32; 21] = [
    0,       // Level 0 (shouldn't normally be used, but makes indexing clean)
    0,       // Level 1
    300,     // Level 2
    900,     // Level 3
    2_700,   // Level 4
    6_500,   // Level 5
    14_000,  // Level 6
    23_000,  // Level 7
    34_000,  // Level 8
    48_000,  // Level 9
    64_000,  // Level 10
    85_000,  // Level 11
    100_000, // Level 12
    120_000, // Level 13
    140_000, // Level 14
    165_000, // Level 15
    195_000, // Level 16
    225_000, // Level 17
    265_000, // Level 18
    305_000, // Level 19
    355_000, // Level 20
];