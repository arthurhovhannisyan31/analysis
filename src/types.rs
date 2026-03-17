use tightness::bound;

bound!(pub MyNonZerou32: u32 where |&v| v > 0);
bound!(pub MyNonZeroi32: i32 where |&v| v != 0);
bound!(pub MyAsciiHexDigit: String where |str| {
  let mut str: &str = &str;

  if str.starts_with('+'){
    str = &str[1..];
  }

  str.chars().all(|c| c.is_ascii_hexdigit())
});

#[cfg(test)]
mod test {
  use crate::types::{MyAsciiHexDigit, MyNonZeroi32, MyNonZerou32};
  use proptest::prelude::*;

  #[test]
  fn test_my_non_zero_u32() {
    assert!(matches!(MyNonZerou32::new(0), Err(_)));
    assert!(matches!(MyNonZerou32::new(1), Ok(_)));
  }

  #[test]
  fn test_my_non_zero_i32() {
    assert!(matches!(MyNonZeroi32::new(-1), Ok(_)));
    assert!(matches!(MyNonZeroi32::new(0), Err(_)));
    assert!(matches!(MyNonZeroi32::new(1), Ok(_)));
  }

  proptest! {
    #[test]
    fn test_my_ascii_hex_digit_ints(a in "[0-9]"){
      assert!(matches!(MyAsciiHexDigit::new(a), Ok(_)))
    }
    #[test]
    fn test_my_ascii_hex_digit_lc_chars(a in "[a-f]"){
      assert!(matches!(MyAsciiHexDigit::new(a), Ok(_)))
    }
    fn test_my_ascii_hex_digit_uc_chars(a in "[A-F]"){
      assert!(matches!(MyAsciiHexDigit::new(a), Ok(_)))
    }
  }
}
