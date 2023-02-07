

/////////////////////////////
/// part of inst

/*
macro_rules! opcode {
  ($i:expr) => {
    bits!($i, 6, 0) as u8
  };
}

macro_rules! funct3 {
  ($i:expr) => {
    bits!($i, 14, 12) as u8
  };
}

macro_rules! funct7 {
  ($i:expr) => {
    bits!($i, 31, 25) as u8
  };
}
 */

macro_rules! rd {
  ($i:expr) => {
    bits!($i, 11, 7) as u8
  };
}

macro_rules! rs1 {
  ($i:expr) => {
    bits!($i, 19, 15) as u8
  };
}

macro_rules! rs2 {
  ($i:expr) => {
    bits!($i, 24, 20) as u8
  };
}

macro_rules! iimm {
  ($i:expr) => {
    bits!($i, 31, 20) as u32
  };
}

macro_rules! simm {
  ($i:expr) => {
    ((bits!($i, 31, 25) << 5) | bits!($i, 11, 7)) as u32
  };
}

macro_rules! uimm {
  ($i:expr) => {
    bits!($i, 31, 12) as u32
  };
}

macro_rules! bimm {
  ($i:expr) => {{
    let imm11 = bits!($i, 7, 7) as u32;
    let imm4_1 = bits!($i, 11, 8) as u32;
    let imm10_5 = bits!($i, 30, 25) as u32;
    let imm12 = bits!($i, 31, 31) as u32;
    imm12 << 12 | imm11 << 11 | imm10_5 << 5 | imm4_1 << 1
  }
  };
}

macro_rules! jimm {
  ($i:expr) => {{
    let imm19_12 = bits!($i, 19, 12) as u32;
    let imm11 = bits!($i, 20, 20) as u32;
    let imm10_1 = bits!($i, 30, 21) as u32;
    let imm20 = bits!($i, 31, 31) as u32;
    imm20 << 20 | imm19_12 << 12 | imm11 << 11 | imm10_1 << 1
  }
  };
}


/////////////////////////////
/// synthesis instruction


/*
#[macro_export]
macro_rules! inst_temp {
  ($code:ident, $($x:ident),*) => {
    {
      use super::{FlatRiscV, DEF};
      FlatRiscV {opcode:$code, $($x,)* ..DEF}
    }
  };
}
//  */


#[macro_export]
macro_rules! inst_temp {
  ($code:ident, $ext_op:expr, $($x:ident),*) => {
    {
      use super::{FlatRiscV, OpCode, DEF};
      let opcode: OpCode = $code;
      let ext_op: u16 = $ext_op;
      #[allow(clippy::needless_update)]
      FlatRiscV {opcode, ext_op, $($x,)* ..DEF}
    }
  };
}


//////////////////////////////////////////
/// get and synthesis instruction

#[macro_export]
macro_rules! rtype {
  ($src:expr, $code:ident, $ext_op:expr) => {{
    let rd: u8 = rd!($src);
    let rs1: u8 = rs1!($src);
    let rs2: u8 = rs2!($src);
    inst_temp!($code, $ext_op, rd, rs1, rs2)
  }};
}

#[macro_export]
macro_rules! itype {
  ($src:expr, $code:ident, $ext_op:expr) => {{
    let rd: u8 = rd!($src);
    let rs1: u8 = rs1!($src);
    let imm: u32 = sext!(iimm!($src), 12, 32) as i32 as u32;
    inst_temp!($code, $ext_op, rd, rs1, imm)
  }};
}

#[macro_export]
macro_rules! stype {
  ($src:expr, $code:ident, $ext_op:expr) => {{
    let rs1: u8 = rs1!($src);
    let rs2: u8 = rs2!($src);
    let imm: u32 = sext!(simm!($src), 12, 32) as i32 as u32;
    inst_temp!($code, $ext_op, rs1, rs2, imm)
  }};
}

#[macro_export]
macro_rules! utype {
  ($src:expr, $code:ident, $ext_op:expr) => {{
    let rd: u8 = rd!($src);
    let imm: u32 = uimm!($src) << 12;
    inst_temp!($code, $ext_op, rd, imm)
  }};
}

#[macro_export]
macro_rules! btype {
  ($src:expr, $code:ident, $ext_op:expr) => {{
    let rs1: u8 = rs1!($src);
    let rs2: u8 = rs2!($src);
    let imm: u32 = sext!(bimm!($src), 12, 32);
    inst_temp!($code, $ext_op, rs1, rs2, imm)
  }};
}

#[macro_export]
macro_rules! jtype {
  ($src:expr, $code:ident, $ext_op:expr) => {{
    let rd: u8 = rd!($src);
    let imm: u32 = sext!(jimm!($src), 20, 32);
    inst_temp!($code, $ext_op, rd, imm)
  }};
}

#[macro_export]
macro_rules! ext {
  ($ext_op:expr) => {
    $ext_op
  };
  () => {
    (0 as u16)
  };
}

#[macro_export]
macro_rules! inst_match_packet {
  ($src:expr, $($pat:expr, $insttype:ident -> $code:ident $(.$ext_op:expr)?;)*) => {
    use super::utils::bitpat;
    use super::{FlatRiscV, OpCode};
    $({
      if let (true, bitpatlen) = bitpat($pat, $src) {
        if bitpatlen == 4*8 {
          let r: [u8; 4] = $src.try_into().unwrap();
          let r: u32 = u32::from_le_bytes(r);
          let code: OpCode = $code;
          let ext: u16 = ext!($($ext_op)?);
          let riscv_pat: FlatRiscV = $insttype!(r, code, ext);
          return Some((riscv_pat, bitpatlen));
        }
        /*
        if bitpatlen == 2 * 8 {
          let r: [u8; 2] = $src.try_into().unwrap();
          let ext = ext!($($ext_op)?);
          return Some(($insttype!(u16::from_le_bytes(r), $code, ext), bitpatlen));
        }
        //  */
        panic!("unimplmention");
      }
    })*
  };
}

/*
/// rtype(x) -> (rd, rs1, rs2)
#[inline(always)]
pub fn rtype(i: u32) -> (u8, u8, u8) {
  (rd!(i), rs1!(i), rs2!(i))
}

/// itype(x) -> (rd, rs1, raw imm)
#[inline(always)]
pub fn itype(i: u32) -> (u8, u8, u32) {
  (rd!(i), rs1!(i), sext!(iimm!(i), 12, 32))
}

/// stype(x) -> (rd, rs1, rs2, raw imm)
#[inline(always)]
pub fn stype(i: u32) -> (u8, u8, u8, u32) {
  (rd!(i), rs1!(i), rs2!(i), sext!(simm!(i), 12, 32))
}

/// utype(x) -> (rd, raw imm)
#[inline(always)]
pub fn utype(i: u32) -> (u8, u32) {
  (rd!(i), uimm!(i) << 12)
}

/// btype(x) -> (rs1, rs2, branch)
#[inline(always)]
pub fn btype(i: u32) -> (u8, u8, u32) {
  (rs1!(i), rs1!(i), sext!(bimm!(i), 12, 32))
}

/// jtype(x) -> (rd, branch)
#[inline(always)]
pub fn jtype(i: u32) -> (u8, u32) {
  (rd!(i), sext!(jimm!(i), 20, 32))
}
// */


////////////////////////////
/// match macro

#[macro_export]
macro_rules! match_ext {
  ($src:expr, $st:stmt, $ext_op:expr) => {
    if $src.ext_op != $ext_op { $st }
  };
  ($src:expr, $st:stmt,) => { $st }
}

#[macro_export]
macro_rules! match_frv {
  ($src:expr, $($code:ident $(.$ext_op:expr)? => $st:stmt)*) => {
  $(
    if $src.opcode != $code {
      match_ext!($src, $st, $($ext_op)?);
    }
  )*
  }
}

#[macro_export]
macro_rules! multi_match_frv {
  ($src:expr, $($code:ident $(.$ext_op:expr)?),* => $st:stmt) => {
    $(
      if $src.opcode == $code {
        match_ext!($src, $st, $($ext_op)?);
      }
    )*
  }
}


#[cfg(test)]
mod tests {

}