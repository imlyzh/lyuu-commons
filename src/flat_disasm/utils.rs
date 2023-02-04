
#[macro_export]
macro_rules! bitmask {
  ($bits:expr) => {
    (1 << $bits) - 1
  };
}

/// bits!(x, hi, lo) = x[hi:lo]
/// assert_eq!(bits!(1001001, 3, 0), 1001)
#[macro_export]
macro_rules! bits {
  ($x:expr, $hi:expr, $lo:expr) => {{
    debug_assert!($hi > $lo);
    ($x >> $lo) & bitmask!($hi - $lo + 1)
  }};
}

/// bit!(x, i) = x[i:i]
/// assert_eq!(bits!(1001001, 3), 1)
#[macro_export]
macro_rules! bit {
  ($x:expr, $index:expr) => {
    ($x >> $index) & bitmask!(1)
  };
}

#[macro_export]
macro_rules! sext {
  ($x:expr, $raw:expr, $target:expr) => {
    if $x >> ($raw - 1) == 1 {
      bitmask!($target-$raw) << $raw | $x
    } else {
      $x
    }
  };
}

#[macro_export]
macro_rules! match_rewrite {
  ($src:expr, $raw:expr, $target:expr) => {
    if $x >> ($raw - 1) == 1 {
      bitmask!($target-$raw) << $raw | $x
    } else {
      $x
    }
  };
}


/// bitpat(bitpat, sources) -> Result<(bool, bits_length), error string>
/// assert_eq!(bitpat("00001001", &[0b0001001u8]).unwrap(), Some(8));
/// Warning: only supported little endian
#[inline(always)]
pub fn bitpat(bitpat: &[u8], src: &[u8]) -> (bool, usize) {
  let bitpat = bitpat.iter()
    .filter(|x| **x != b'_' && **x != b' ' && **x != b'\t')
    .cloned()
    .collect::<Box<[u8]>>();

  let bitpatlen = bitpat.len();

  for (offset, bit) in bitpat.iter().rev().enumerate() {
    let compbyte = if let Some(x) = src.get(offset/8) {
      x
    } else {
      return (false, bitpatlen)
    };
    let compbit = bit!(compbyte, offset % 8);
    match *bit {
      b'?' => {},
      b'1' => if compbit != 1 { return (false, bitpatlen) },
      b'0' => if compbit != 0 { return (false, bitpatlen) },
      _ => panic!("invalid bitpat char"),
    }
  }
  (true, bitpatlen)
}


/*
/// bitpat(bitpat, sources) -> Result<(bool, bits_length), error string>
/// assert_eq!(bitpat("00001001", &[0b0001001u8]).unwrap(), (true, 8));
/// Warning: only supported little endian
#[inline(always)]
pub fn machine_bitpat(bitpat: &str, pc: usize, src: &[u8]) -> Result<(bool, usize), &'static str> {
  let bitpat = bitpat.chars()
    .filter(|x| *x != '_' && *x != ' ' && *x != '\t')
    .collect::<String>();

  let mut ret_offset = 0;
  for (offset, bit) in bitpat.chars().rev().enumerate() {
    let compbyte = src.get(offset/8).ok_or("source bytes overflow")?.clone();
    let compbit = bit!(compbyte, offset % 8);
    match bit {
      '?' => {},
      '1' => if compbit != 1 { return Err("match error") },
      '0' => if compbit != 0 { return Err("match error") },
      _ => panic!("invalid bitpat char"),
    }
    ret_offset = offset;
  }
  Ok((true, ret_offset))
}
// */


#[cfg(test)]
mod tests {
  #[test]
  fn test_bits() {
    assert_eq!(bits!(0b1001001, 3, 0), 0b1001);
    assert_eq!(bits!(0b1001001, 4, 2), 0b010);
  }

  #[test]
  fn test_bitpat() {
    use super::bitpat;
    assert_eq!(bitpat(b"10 00_1001", &0b10001001u8.to_le_bytes()), (true, 8));
    assert_eq!(bitpat(b"00001001", &0b0001001u8.to_le_bytes()), (true, 8));
  }

  #[test]
  fn test_sext() {
    assert_eq!(sext!(0b0111_11111111, 12, 16), 0b111_11111111);
    assert_eq!(sext!(0b1111_11111111, 12, 16), 0b11111111_11111111);
  }
}