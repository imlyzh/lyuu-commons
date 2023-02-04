// use crate::{inst_match};
use super::{OpCode::*, FlatRiscV};


macro_rules! flag_gen {
  ($name:ident, $value:expr) => {
  #[allow(non_upper_case_globals)]
  pub const $name: u16 = $value;
  };
}

flag_gen!(eq, 0b000);
flag_gen!(ne, 0b001);
flag_gen!(lt, 0b100);
flag_gen!(ge, 0b101);
flag_gen!(ltu, 0b110);
flag_gen!(geu, 0b111);

flag_gen!(b, 0b000);
flag_gen!(h, 0b001);
flag_gen!(w, 0b010);
flag_gen!(bu, 0b100);
flag_gen!(hu, 0b101);

//////////////////////////////
/// impl

pub fn flat_disasm(src: &[u8]) -> Option<(FlatRiscV, usize)> {
  inst_match!(src,
    b"????????????????????_?????_0110111", utype -> lui;
    b"????????????????????_?????_0010111", utype -> auipc;
    b"????????????????????_?????_1101111", jtype -> jal;
    b"?????????????????000_?????_1100111", jtype -> jalr;
    b"?????????????????000_?????_1100011", btype -> br.eq;
    b"?????????????????001_?????_1100011", btype -> br.ne;
    b"?????????????????100_?????_1100011", btype -> br.lt;
    b"?????????????????101_?????_1100011", btype -> br.ge;
    b"?????????????????110_?????_1100011", btype -> br.ltu;
    b"?????????????????111_?????_1100011", btype -> br.geu;
    b"?????????????????000_?????_0000011", itype -> load.b;
    b"?????????????????001_?????_0000011", itype -> load.h;
    b"?????????????????010_?????_0000011", itype -> load.w;
    b"?????????????????100_?????_0000011", itype -> load.bu;
    b"?????????????????101_?????_0000011", itype -> load.hu;
    b"?????????????????000_?????_0100011", stype -> store.b;
    b"?????????????????001_?????_0100011", stype -> store.h;
    b"?????????????????010_?????_0100011", stype -> store.w;
  );
}


mod test {
  #[test]
  fn test1() {
    use super::flat_disasm;
    let src = 0b11111111111111111111_00000_0110111_u32.to_le_bytes();
    let r = flat_disasm(&src).unwrap();
    println!("out: {:?}", r);
  }
}