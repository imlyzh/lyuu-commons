use lyuu_commons::flat_disasm::utils::bitpat;


fn main() {
  bitpat(b"10 00_1001", &0b10001001u8.to_le_bytes());
}