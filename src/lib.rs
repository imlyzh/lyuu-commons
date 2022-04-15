use crate::isa::riscv::reg::CSR_MAP;

pub mod isa;


#[test]
fn test() {
    println!("{:?}", CSR_MAP.get(&0x0c01));
}