

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reg(pub u8);

pub type Rd = Reg;
pub type Rs1 = Reg;
pub type Rs2 = Reg;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Csr(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Imm(pub u32);

pub type Shamt = Imm;

pub type Zimm = Imm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pred(pub u8);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Succ(pub u8);


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
    Addi    = 0b0000,
    Slti    = 0b0010,
    Sltiu   = 0b0011,
    Xori    = 0b0100,
    Ori     = 0b0110,
    Andi    = 0b0111,
    Slli    = 0b0001,
    Srli    = 0b0101,
    Srai    = 0b1101,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IsSra(pub bool);


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

pub enum RiscV {
    // rvi
    Lui(Rd, Imm),
    Auipc(Rd, Imm),
    Jal(Rd, Imm),
    Jalr(Rd, Rs1, Imm),
    Branch(BrType, Rs1, Rs2, Imm),
    Load(LoadType, Rd, Rs1, Imm),
    Store(StoreType, Rs1, Rs2, Imm),
    OpI(IOpType, Rd, Rs1, Imm),
    Op(OpType, Rd, Rs1, Rs2),
    Fence(IsFenceI, Pred, Succ),
    EOp(EOpType),
    CsrOp(CsrOpType, Rd, Rs1, Csr),
    CsrOpI(CsrOpType, Rd, Zimm, Csr),
}