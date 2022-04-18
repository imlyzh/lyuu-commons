pub mod inst_binary;
pub mod bare;
pub mod reg;


use std::fmt::Display;

use self::reg::CSR_MAP;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg(pub u8);

impl Reg {
    pub fn new(value: u8) -> Reg {
        assert!(value <= 0b11111);
        Reg(value)
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x{}", self.0)
    }
}

pub type Rd = Reg;
pub type Rs1 = Reg;
pub type Rs2 = Reg;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Csr(pub u16);

impl Display for Csr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = CSR_MAP.get(&self.0.into()).cloned() {
            write!(f, "{}", r)
        } else {
            write!(f, "{}", self.0)
        }
            // .map_or_else(|| self.0.to_string(), identity);
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub type Imm8 = u8;
pub type Imm32 = u32;

pub type Immi8 = i8;
pub type Immi16 = i16;
pub type Immi32 = i32;

pub type Shamt = Imm8;

pub type Zimm = Imm8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pred(pub u8);

impl Display for Pred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Succ(pub u8);

impl Display for Succ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BrType {
    Eq  = 0b000,
    Ne  = 0b001,
    Lt  = 0b100,
    Ge  = 0b101,
    Ltu = 0b110,
    Geu = 0b111,
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LoadType {
    Byte  = 0b000,
    Half  = 0b001,
    Word  = 0b010,
    Double = 0b011,
    ByteU = 0b100,
    HalfU = 0b101,
    WordU = 0b110,
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum StoreType {
    Byte  = 0b000,
    Half  = 0b001,
    Word  = 0b010,
    Double = 0b011,
}


/*
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IOpType {
    Slti    = 0b0010,
    Addi    = 0b0000,
    Sltiu   = 0b0011,
    Xori    = 0b0100,
    Ori     = 0b0110,
    Andi    = 0b0111,
    Slli    = 0b0001,
    Srli    = 0b0101,
    Srai    = 0b1101,
}
 */


#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpType {
    Add     = 0b0000,
    Sub     = 0b1000,
    Sll     = 0b0001,
    Slt     = 0b0010,
    Sltu    = 0b0011,
    Xor     = 0b0100,
    Srl     = 0b0101,
    Sra     = 0b1101,
    Or      = 0b0110,
    And     = 0b0111,
}

/*
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpWType {
    Add     = 0b0000,
    Sub     = 0b1000,
    Sll     = 0b0001,
    Srl     = 0b0101,
    Sra     = 0b1101,
}
// */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IsFenceI(pub bool);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EOpType {
    Call    = 0b0,
    Break   = 0b1
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CsrOpType {
    Rw = 0b001,
    Rs = 0b010,
    Rc = 0b011,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiscV {
    // rvi
    Lui(Rd, Imm32),
    Auipc(Rd, Imm32),
    Jal(Rd, Immi32),
    Jalr(Rd, Rs1, Immi16),
    Branch(BrType, Rs1, Rs2, Immi16),
    Load(LoadType, Rd, Rs1, Immi16),
    Store(StoreType, Rs1, Rs2, Immi16),
    OpI(OpType, Rd, Rs1, Immi16),
    OpIW(OpType, Rd, Rs1, Immi16),
    Op(OpType, Rd, Rs1, Rs2),
    OpW(OpType, Rd, Rs1, Rs2),
    Fence(IsFenceI, Pred, Succ),
    EOp(EOpType),
    CsrOp(CsrOpType, Rd, Rs1, Csr),
    CsrOpI(CsrOpType, Rd, Zimm, Csr),
}

impl Display for RiscV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiscV::Lui(rd, imm) => write!(f, "lui\t{}, {}", rd, imm),
            RiscV::Auipc(rd, imm) => write!(f, "auipc\t{}, {}", rd, imm),
            RiscV::Jal(rd, offset) => write!(f, "jal\t{}, {}", rd, offset),
            RiscV::Jalr(rd, rs1, offset) => write!(f, "jalr\t{}, {}({})", rd, offset, rs1),

            RiscV::Branch(BrType::Eq, rs1, rs2, offset) => write!(f, "beq\t{}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Ne, rs1, rs2, offset) => write!(f, "bne\t{}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Lt, rs1, rs2, offset) => write!(f, "blt\t{}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Ge, rs1, rs2, offset) => write!(f, "bge\t{}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Ltu, rs1, rs2, offset) => write!(f, "bltu\t{}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Geu, rs1, rs2, offset) => write!(f, "bgeu\t{}, {}, {}", rs1, rs2, offset),

            RiscV::Load(LoadType::Byte, rd, rs1, offset) => write!(f, "lb\t{}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::Half, rd, rs1, offset) => write!(f, "lh\t{}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::Word, rd, rs1, offset) => write!(f, "lw\t{}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::Double, rd, rs1, offset) => write!(f, "ld\t{}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::ByteU, rd, rs1, offset) => write!(f, "lbu\t{}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::HalfU, rd, rs1, offset) => write!(f, "lhu\t{}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::WordU, rd, rs1, offset) => write!(f, "lwu\t{}, {}({})", rd, offset, rs1),

            RiscV::Store(StoreType::Byte, rs1, rs2, imm) => write!(f, "sb\t{}, {}({})", rs1, imm, rs2),
            RiscV::Store(StoreType::Half, rs1, rs2, imm) => write!(f, "sh\t{}, {}({})", rs1, imm, rs2),
            RiscV::Store(StoreType::Word, rs1, rs2, imm) => write!(f, "sw\t{}, {}({})", rs1, imm, rs2),
            RiscV::Store(StoreType::Double, rs1, rs2, imm) => write!(f, "sd\t{}, {}({})", rs1, imm, rs2),

            RiscV::OpI(OpType::Add, rd, rs1, imm) => write!(f, "addi\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Slt, rd, rs1, imm) => write!(f, "slti\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Sltu, rd, rs1, imm) => write!(f, "sltiu\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Xor, rd, rs1, imm) => write!(f, "xori\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Or, rd, rs1, imm) => write!(f, "ori\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::And, rd, rs1, imm) => write!(f, "andi\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Sll, rd, rs1, imm) => write!(f, "slli\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Srl, rd, rs1, imm) => write!(f, "srli\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpI(OpType::Sra, rd, rs1, imm) => write!(f, "srai\t{}, {}, {}", rd, rs1, imm),

            RiscV::OpIW(OpType::Add, rd, rs1, imm) => write!(f, "addiw\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpIW(OpType::Sll, rd, rs1, imm) => write!(f, "slliw\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpIW(OpType::Srl, rd, rs1, imm) => write!(f, "srliw\t{}, {}, {}", rd, rs1, imm),
            RiscV::OpIW(OpType::Sra, rd, rs1, imm) => write!(f, "sraiw\t{}, {}, {}", rd, rs1, imm),

            RiscV::Op(OpType::Add, rd, rs1, rs2) => write!(f, "add\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sub, rd, rs1, rs2) => write!(f, "sub\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sll, rd, rs1, rs2) => write!(f, "sll\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Slt, rd, rs1, rs2) => write!(f, "slt\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sltu, rd, rs1, rs2) => write!(f, "sltu\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Xor, rd, rs1, rs2) => write!(f, "xor\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Srl, rd, rs1, rs2) => write!(f, "srl\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sra, rd, rs1, rs2) => write!(f, "sra\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Or, rd, rs1, rs2) => write!(f, "or\t{}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::And, rd, rs1, rs2) => write!(f, "and\t{}, {}, {}", rd, rs1, rs2),

            RiscV::OpW(OpType::Add, rd, rs1, rs2) => write!(f, "addw\t{}, {}, {}", rd, rs1, rs2),
            RiscV::OpW(OpType::Sub, rd, rs1, rs2) => write!(f, "subw\t{}, {}, {}", rd, rs1, rs2),
            RiscV::OpW(OpType::Sll, rd, rs1, rs2) => write!(f, "sllw\t{}, {}, {}", rd, rs1, rs2),
            RiscV::OpW(OpType::Srl, rd, rs1, rs2) => write!(f, "srlw\t{}, {}, {}", rd, rs1, rs2),
            RiscV::OpW(OpType::Sra, rd, rs1, rs2) => write!(f, "sraw\t{}, {}, {}", rd, rs1, rs2),

            RiscV::Fence(IsFenceI(false), pred, succ) => write!(f, "fence.i\t{}, {}", pred, succ),
            RiscV::Fence(IsFenceI(true), _, _) => write!(f, "fence"),

            RiscV::EOp(EOpType::Call) => write!(f, "ecall"),
            RiscV::EOp(EOpType::Break) => write!(f, "ebreak"),

            RiscV::CsrOpI(CsrOpType::Rw, rd, rs1, csr) => write!(f, "csrrwi\t{}, {}, {}", rd, rs1, csr),
            RiscV::CsrOpI(CsrOpType::Rs, rd, rs1, csr) => write!(f, "csrrsi\t{}, {}, {}", rd, rs1, csr),
            RiscV::CsrOpI(CsrOpType::Rc, rd, rs1, csr) => write!(f, "csrrci\t{}, {}, {}", rd, rs1, csr),

            RiscV::CsrOp(CsrOpType::Rw, rd, rs1, csr) => write!(f, "csrrw\t{}, {}, {}", rd, rs1, csr),
            RiscV::CsrOp(CsrOpType::Rs, rd, rs1, csr) => write!(f, "csrrs\t{}, {}, {}", rd, rs1, csr),
            RiscV::CsrOp(CsrOpType::Rc, rd, rs1, csr) => write!(f, "csrrc\t{}, {}, {}", rd, rs1, csr),

            _ => panic!("is not supported"),
        }
    }
}