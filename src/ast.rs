#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Mov(Mov),
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Pop(Pop),
    Call(Call),
    Allow(Allow),
}

#[derive(Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Register {
    R1 = 0x01,
    R2 = 0x02,
    R3 = 0x03,
    R4 = 0x04,
    R5 = 0x05,
    R6 = 0x06,
    R7 = 0x07,
    R8 = 0x08,
    R9 = 0x09,
    R10 = 0x0A,
    R11 = 0x0B,
    R12 = 0x0C,
    R13 = 0x0D,
    R14 = 0x0E,
    R15 = 0x0F,
    Rbp = 0x10,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0b00001 => Register::R1,
            0b00010 => Register::R2,
            0b00011 => Register::R3,
            0b00100 => Register::R4,
            0b00101 => Register::R5,
            0b00110 => Register::R6,
            0b00111 => Register::R7,
            0b01000 => Register::R8,
            0b01001 => Register::R9,
            0b01010 => Register::R10,
            0b01011 => Register::R11,
            0b01100 => Register::R12,
            0b01101 => Register::R13,
            0b01110 => Register::R14,
            0b01111 => Register::R15,
            0b10000 => Register::Rbp,
            _ => panic!("invalid register"),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Mov(pub MemoryFetching, pub Expr);

#[derive(Clone, Debug, PartialEq)]
pub struct Add(pub MemoryFetching, pub Expr);

#[derive(Clone, Debug, PartialEq)]
pub struct Sub(pub MemoryFetching, pub Expr);

#[derive(Clone, Debug, PartialEq)]
pub struct Mul(pub MemoryFetching);

#[derive(Clone, Debug, PartialEq)]
pub struct Div(pub MemoryFetching, pub Expr);

#[derive(Clone, Debug, PartialEq)]
pub struct Pop(pub MemoryFetching);

#[derive(Clone, Debug, PartialEq)]
pub struct Call(pub String);

#[derive(Clone, Debug, PartialEq)]
pub struct Allow(pub Expr, pub Expr);

#[derive(Clone, Debug, PartialEq)]
pub struct Label {
    pub name: String,
    pub program: Vec<Command>,
}

impl Label {
    pub fn new(name: String, program: Vec<Command>) -> Self {
        Self { name, program }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub labels: Vec<Label>,
}

impl Program {
    pub fn new(labels: Vec<Label>) -> Self {
        Self { labels }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Int(i32),
    Label(String),
    Memory(MemoryFetching),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MemoryFetching {
    Addr(usize),
    Register(Register),
}
