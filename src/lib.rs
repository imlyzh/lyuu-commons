#[allow(unused_imports)]
use crate::isa::riscv::reg::CSR_MAP;

pub mod utils;
pub mod isa;
pub mod disassembly;


#[test]
fn test() {
    println!("{:?}", CSR_MAP.get(&0x0c01));
}