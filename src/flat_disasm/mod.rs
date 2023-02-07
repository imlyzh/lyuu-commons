#[macro_use]
pub mod utils;
#[macro_use]
pub mod riscv_utils;
pub mod disasm;



////////////////////////////////
/// struct define


#[allow(non_camel_case_types)]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
  invalid = 0,
  lui,
  auipc,
  jal,
  jalr,
  br,
  load,
  store,
  iop,
  op,
  fence,
  excep,
  csr,
}

use std::fmt::Display;

use OpCode::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlatRiscV {
  pub opcode: OpCode,
  pub ext_op: u16,
  pub rd: u8,
  pub rs1: u8,
  pub rs2: u8,
  pub imm: u32, // imm and shamt
}


impl FlatRiscV {
  pub fn post_process(&mut self, is_32bit: bool) {
    if is_32bit {
      multi_match_frv!(self,
        iop.sll,
        iop.srl,
        iop.sra => { self.imm = bits!(self.imm, 4, 0); }
      );
    } else {
      multi_match_frv!(self,
        iop.sll,
        iop.srl,
        iop.sra => { self.imm = bits!(self.imm, 5, 0); }
      );
      multi_match_frv!(self,
        iop.sllw,
        iop.srlw,
        iop.sraw  => { self.imm = bits!(self.imm, 4, 0); }
      );
    }
    multi_match_frv!(self,
      op,
      excep.ret => { self.imm = 0; }
    );
  }
}


impl Display for FlatRiscV {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.imm == 0 {
      write!(f, "{:?}.{:b}  x{}, x{}, x{}", self.opcode, self.ext_op, self.rd, self.rs1, self.rs2)
    } else {
      write!(f, "{:?}.{:b}  x{}, x{}, x{}, 0b{:b}", self.opcode, self.ext_op, self.rd, self.rs1, self.rs2, self.imm)
    }
  }
}

//////////////////////////////
// operator number gen

macro_rules! flag_gen {
  ($name:ident, $value:expr) => {
  #[allow(non_upper_case_globals)]
  pub const $name: u16 = $value;
  };
}

// cmp flag
flag_gen!(eq  , 0b000);
flag_gen!(ne  , 0b001);
flag_gen!(lt  , 0b100);
flag_gen!(ge  , 0b101);
flag_gen!(ltu , 0b110);
flag_gen!(geu , 0b111);

// load store flag
flag_gen!(b , 0b000);
flag_gen!(h , 0b001);
flag_gen!(w , 0b010);
flag_gen!(d , 0b011);
flag_gen!(bu, 0b100);
flag_gen!(hu, 0b101);
flag_gen!(wu, 0b110);

// operator
flag_gen!(add , 0b00000);
flag_gen!(addw, 0b00001);
flag_gen!(sub , 0b00010);
flag_gen!(subw, 0b00011);
flag_gen!(sll , 0b00100);
flag_gen!(sllw, 0b00101);
flag_gen!(slt , 0b01000);
flag_gen!(sltu, 0b01100);
flag_gen!(xor , 0b10000);
flag_gen!(srl , 0b10100);
flag_gen!(srlw, 0b10101);
flag_gen!(sra , 0b10110);
flag_gen!(sraw, 0b10111);
flag_gen!(or  , 0b11000);
flag_gen!(and , 0b11100);

// fence.i
flag_gen!(i, 0b1);

// ecall
flag_gen!(call, 0b0);
flag_gen!(ret, 0b1);  // break, break is rust keyword


// csr
flag_gen!(rw  , 0b001);
flag_gen!(rs  , 0b010);
flag_gen!(rc  , 0b011);
flag_gen!(rwi , 0b101);
flag_gen!(rsi , 0b110);
flag_gen!(rci , 0b111);


// default flatrv
pub const DEF: FlatRiscV = FlatRiscV {
  opcode: invalid,
  ext_op: 0,
  rd: 0,
  rs1: 0,
  rs2: 0,
  imm: 0,
};
