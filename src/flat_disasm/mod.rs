#[macro_use]
pub mod utils;
#[macro_use]
pub mod riscv_utils;
pub mod disasm;



////////////////////////////////
/// struct define


#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum OpCode {
  invalid = 0,
  lui,
  auipc,
  jal,
  jalr,
  br,
  load,
  store,
  immop,
}

use OpCode::*;


#[derive(Debug, Clone, Copy)]
pub struct FlatRiscV {
  pub opcode: OpCode,
  pub ext_op: u16,
  pub rd: u8,
  pub rs1: u8,
  pub rs2: u8,
  pub imm: u32, // imm and shamt
}

// default flatrv
pub const DEF: FlatRiscV = FlatRiscV {
  opcode: invalid,
  ext_op: 0,
  rd: 0,
  rs1: 0,
  rs2: 0,
  imm: 0,
};
