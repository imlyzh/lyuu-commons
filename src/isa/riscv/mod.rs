use std::fmt::Display;

pub mod inst_binary;
pub mod bare;


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
        write!(f, "csr{}", self.0)
    }
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub type Imm8 = u8;
pub type Imm32 = u32;

pub type Immi8 = i8;
pub type Immi16 = i16;
pub type Immi32 = i32;

pub type Shamt = Imm32;

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
    OpI(IOpType, Rd, Rs1, Immi16),
    Op(OpType, Rd, Rs1, Rs2),
    Fence(IsFenceI, Pred, Succ),
    EOp(EOpType),
    CsrOp(CsrOpType, Rd, Rs1, Csr),
    CsrOpI(CsrOpType, Rd, Zimm, Csr),
}

impl Display for RiscV {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiscV::Lui(rd, imm) => write!(f, "lui {}, {}", rd, imm),
            RiscV::Auipc(rd, imm) => write!(f, "auipc {}, {}", rd, imm),
            RiscV::Jal(rd, offset) => write!(f, "jal {}, {}", rd, offset),
            RiscV::Jalr(rd, rs1, offset) => write!(f, "jalr {}, {}({})", rd, offset, rs1),

            RiscV::Branch(BrType::Eq, rs1, rs2, offset) => write!(f, "beq {}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Ne, rs1, rs2, offset) => write!(f, "bne {}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Lt, rs1, rs2, offset) => write!(f, "blt {}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Ge, rs1, rs2, offset) => write!(f, "bge {}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Ltu, rs1, rs2, offset) => write!(f, "bltu {}, {}, {}", rs1, rs2, offset),
            RiscV::Branch(BrType::Geu, rs1, rs2, offset) => write!(f, "bgeu {}, {}, {}", rs1, rs2, offset),

            RiscV::Load(LoadType::Byte, rd, rs1, offset) => write!(f, "lb {}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::Half, rd, rs1, offset) => write!(f, "lh {}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::Word, rd, rs1, offset) => write!(f, "lw {}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::Double, rd, rs1, offset) => write!(f, "ld {}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::ByteU, rd, rs1, offset) => write!(f, "lbu {}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::HalfU, rd, rs1, offset) => write!(f, "lhu {}, {}({})", rd, offset, rs1),
            RiscV::Load(LoadType::WordU, rd, rs1, offset) => write!(f, "lwu {}, {}({})", rd, offset, rs1),

            RiscV::Store(StoreType::Byte, rs1, rs2, imm) => write!(f, "sb {}, {}({})", rs1, imm, rs2),
            RiscV::Store(StoreType::Half, rs1, rs2, imm) => write!(f, "sh {}, {}({})", rs1, imm, rs2),
            RiscV::Store(StoreType::Word, rs1, rs2, imm) => write!(f, "sw {}, {}({})", rs1, imm, rs2),
            RiscV::Store(StoreType::Double, rs1, rs2, imm) => write!(f, "sd {}, {}({})", rs1, imm, rs2),

            RiscV::OpI(IOpType::Addi, rd, rs1, imm) => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Slti, rd, rs1, imm) => write!(f, "slti {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Sltiu, rd, rs1, imm) => write!(f, "sltiu {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Xori, rd, rs1, imm) => write!(f, "xori {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Ori, rd, rs1, imm) => write!(f, "ori {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Andi, rd, rs1, imm) => write!(f, "andi {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Slli, rd, rs1, imm) => write!(f, "slli {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Srli, rd, rs1, imm) => write!(f, "srli {}, {}, {}", rd, rs1, imm),
            RiscV::OpI(IOpType::Srai, rd, rs1, imm) => write!(f, "srai {}, {}, {}", rd, rs1, imm),

            RiscV::Op(OpType::Add, rd, rs1, rs2) => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sub, rd, rs1, rs2) => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sll, rd, rs1, rs2) => write!(f, "sll {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Slt, rd, rs1, rs2) => write!(f, "slt {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sltu, rd, rs1, rs2) => write!(f, "sltu {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Xor, rd, rs1, rs2) => write!(f, "xor {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Srl, rd, rs1, rs2) => write!(f, "srl {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Sra, rd, rs1, rs2) => write!(f, "sra {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::Or, rd, rs1, rs2) => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            RiscV::Op(OpType::And, rd, rs1, rs2) => write!(f, "and {}, {}, {}", rd, rs1, rs2),

            RiscV::Fence(IsFenceI(false), pred, succ) => write!(f, "fence.i {}, {}", pred, succ),
            RiscV::Fence(IsFenceI(true), _, _) => write!(f, "fence"),

            RiscV::EOp(EOpType::Call) => write!(f, "ecall"),
            RiscV::EOp(EOpType::Break) => write!(f, "ebreak"),

            RiscV::CsrOpI(CsrOpType::Rw, rd, rs1, csr) => write!(f, "csrrwi {}, {}, {}", rd, rs1, csr),
            RiscV::CsrOpI(CsrOpType::Rs, rd, rs1, csr) => write!(f, "csrrsi {}, {}, {}", rd, rs1, csr),
            RiscV::CsrOpI(CsrOpType::Rc, rd, rs1, csr) => write!(f, "csrrci {}, {}, {}", rd, rs1, csr),

            RiscV::CsrOp(CsrOpType::Rw, rd, rs1, csr) => write!(f, "csrrw {}, {}, {}", rd, rs1, csr),
            RiscV::CsrOp(CsrOpType::Rs, rd, rs1, csr) => write!(f, "csrrs {}, {}, {}", rd, rs1, csr),
            RiscV::CsrOp(CsrOpType::Rc, rd, rs1, csr) => write!(f, "csrrc {}, {}, {}", rd, rs1, csr),
        }
    }
}