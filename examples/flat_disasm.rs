use lyuu_commons::flat_disasm::disasm::flat_disasm;


fn main() {
  let src = 0b11111111111111111111_00000_0110111_u32.to_le_bytes();
  let (r, _next) = flat_disasm(&src).unwrap();
  println!("out: {}", r);
}
