use tightness::bound;

bound!(pub MyNonZeroU32: u32 where |&v| v > 0);
bound!(pub MyNonZeroi32: i32 where |&v| v != 0);
bound!(pub MyAsciiHexDigit: String where |str| str.chars().all(|c| c.is_ascii_hexdigit()));
