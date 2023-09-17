use crate::ast::*;
use crate::MODE;
use std::collections::HashMap;
use std::fmt::{Binary, LowerHex};

pub static VOID: u8 = 0x0;
pub static REG: u8 = 0x1;
pub static INT: u8 = 0x2;
pub static LABEL: u8 = 0x3;
pub static MEM: u8 = 0x4;
pub static MOV: u8 = 0x11;
pub static ADD: u8 = 0x12;
pub static SUB: u8 = 0x13;
pub static MUL: u8 = 0x14;
pub static DIV: u8 = 0x15;
pub static POP: u8 = 0x16;
pub static CALL: u8 = 0x17;
pub static ALLOW: u8 = 0x18;
pub static NOP: u8 = 0x19;
pub static JUMP: u8 = 0x20;
pub static RET: u8 = 0x21;


#[derive(Clone, Debug, PartialEq)]
pub struct MachineCodeInstruction {
    pub label: [u8; 4],
    pub instr: u8,
    pub operand1_type: u8,
    pub operand1: [u8; 4],
    pub operand2_type: u8,
    pub operand2: [u8; 4],
}

impl MachineCodeInstruction {
    pub fn new(
        label: [u8; 4],
        instr: u8,
        operand1_type: u8,
        operand1: [u8; 4],
        operand2_type: u8,
        operand2: [u8; 4],
    ) -> Self {
        Self {
            label,
            instr,
            operand1_type,
            operand1,
            operand2_type,
            operand2,
        }
    }

    pub fn binary_string(&self) -> String {
        let space = if MODE.is_debug() { " " } else { "" };
        let instr_addr_bin = self
            .label
            .iter()
            .map(|x| add_zero(format!("{:b}", x), 8))
            .collect::<Vec<String>>()
            .join(space);
        let instr_bin = add_zero(format!("{:b}", self.instr), 7);
        let operand1_type_bin = add_zero(format!("{:b}", self.operand1_type), 4);
        let operand1_bin = self
            .operand1
            .iter()
            .map(|x| add_zero(format!("{:b}", x), 8))
            .collect::<Vec<String>>()
            .join(space);
        let operand2_type_bin = add_zero(format!("{:b}", self.operand2_type), 4);
        let operand2_bin = self
            .operand2
            .iter()
            .map(|x| add_zero(format!("{:b}", x), 8))
            .collect::<Vec<String>>()
            .join(space);

        vec![
            instr_addr_bin,
            instr_bin,
            operand1_type_bin,
            operand1_bin,
            operand2_type_bin,
            operand2_bin,
        ]
        .join(space)
    }

    pub fn hex_string(&self) -> String {
        let space = if MODE.is_debug() { " " } else { "" };
        let instr_addr_mem_hex = self
            .label
            .iter()
            .map(|x| add_zero(format!("{:x}", x), 4))
            .collect::<Vec<String>>()
            .join(space);
        let instr_hex = add_zero(format!("{:x}", self.instr), 4);
        let operand1_type_hex = add_zero(format!("{:x}", self.operand1_type), 4);

        let operand1_hex = self
            .operand1
            .iter()
            .map(|x| add_zero(format!("{:x}", x), 4))
            .collect::<Vec<String>>()
            .join(space);
        let operand2_type_hex = add_zero(format!("{:x}", self.operand2_type), 4);

        let operand2_hex = self
            .operand2
            .iter()
            .map(|x| add_zero(format!("{:x}", x), 4))
            .collect::<Vec<String>>()
            .join(space);

        vec![
            instr_addr_mem_hex,
            instr_hex,
            operand1_type_hex,
            operand1_hex,
            operand2_type_hex,
            operand2_hex,
        ]
        .join(space)
    }

    pub fn to_bytecode(&self) -> Vec<u8> {
        let mut bytecode = vec![];
        bytecode.extend(self.label);
        bytecode.extend(self.instr.to_le_bytes());
        bytecode.extend(self.operand1_type.to_le_bytes());
        bytecode.extend(self.operand1);
        bytecode.extend(self.operand2_type.to_le_bytes());
        bytecode.extend(self.operand2);
        bytecode
    }

    pub fn from_bytecode(_bytecode: Vec<u8>) -> Self {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct MachineCode {
    code: Vec<MachineCodeInstruction>,
}

impl MachineCode {
    pub fn new(code: Vec<MachineCodeInstruction>) -> Self {
        Self { code }
    }

    pub fn push(&mut self, instr: MachineCodeInstruction) {
        self.code.push(instr);
    }

    pub fn extend<I: IntoIterator<Item = MachineCodeInstruction>>(&mut self, instrs: I) {
        self.code.extend(instrs);
    }

    pub fn last_mut(&mut self) -> Option<&mut MachineCodeInstruction> {
        self.code.last_mut()
    }

    pub fn contains_addr_instr(&self, addr: u32) -> bool {
        self.code
            .iter()
            .filter(|x| u32::from_le_bytes(x.label) == addr)
            .count()
            != 0
    }
}

impl Binary for MachineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for byte in &self.code {
            string.push_str(&byte.binary_string());
            string.push('\n');
        }

        write!(f, "{}", string)
    }
}

impl LowerHex for MachineCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for byte in &self.code {
            string.push_str(&byte.hex_string());
            string.push('\n');
        }

        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
pub struct MachineCodeCompiler {
    pub program: Program,
    pub machine_code: MachineCode,
    labels: HashMap<String, u32>,
    current_label: u32,
}

impl MachineCodeCompiler {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            machine_code: MachineCode::new(vec![]),
            labels: HashMap::new(),
            current_label: 1,
        }
    }

    pub fn compile(&mut self) -> MachineCode {
        self.setup_labels(self.program.labels.clone());
        self.labels_compiler(self.program.labels.clone());
        self.machine_code.push(MachineCodeInstruction::new(
            self.int_to_bytes(1),
            NOP,
            VOID,
            Default::default(),
            VOID,
            Default::default(),
        ));
        self.machine_code.clone()
    }

    pub fn setup_labels(&mut self, labels: Vec<Label>) {
        labels.iter().fold(1, move |acc, x| {
            self.labels.insert(x.name.clone(), acc);
            acc + x.program.len() as u32
        });
    }

    pub fn labels_compiler(&mut self, labels: Vec<Label>) {
        labels.iter().for_each(|x| {
            self.current_label = self.labels.get(&x.name).cloned().unwrap();
            self.command_compiler(x.program.clone());
        })
    }

    pub fn command_compiler(&mut self, commands: Vec<Command>) {
        for cmd in commands.clone() {
            match cmd {
                Command::Mov(mov) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(mov.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(mov.1);

                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        MOV,
                        operand1_type,
                        operand1,
                        operand2_type,
                        operand2,
                    ));
                }
                Command::Add(add) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(add.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(add.1);
                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        ADD,
                        operand1_type,
                        operand1,
                        operand2_type,
                        operand2,
                    ));
                }
                Command::Sub(sub) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(sub.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(sub.1);

                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        SUB,
                        operand1_type,
                        operand1,
                        operand2_type,
                        operand2,
                    ));
                }
                Command::Mul(mul) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(mul.0);

                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        MUL,
                        operand1_type,
                        operand1,
                        VOID,
                        Default::default(),
                    ));
                }
                Command::Div(div) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(div.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(div.1);

                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        DIV,
                        operand1_type,
                        operand1,
                        operand2_type,
                        operand2,
                    ));
                }
                Command::Pop(pop) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(pop.0);

                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        POP,
                        operand1_type,
                        operand1,
                        VOID,
                        Default::default(),
                    ));
                }
                Command::Call(call) => {
                    let label = *self.labels.get(&call.0).unwrap();
                    let n = self.int_to_bytes(label);
                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        CALL,
                        LABEL,
                        n,
                        VOID,
                        Default::default(),
                    ));
                    self.current_label = label;
                }
                Command::Allow(allow) => {
                    let (operand1_type, operand1) = self.expr_to_bytes(allow.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(allow.1);

                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        ALLOW,
                        operand1_type,
                        operand1,
                        operand2_type,
                        operand2,
                    ));
                }
                Command::Ret(_) => {
                    self.machine_code.push(MachineCodeInstruction::new(
                        self.int_to_bytes(self.current_label),
                        RET,
                        VOID,
                        Default::default(),
                        VOID,
                        Default::default(),
                    ));
                }
            }
        }
    }

    pub fn register_to_bytes(&self, reg: Register) -> [u8; 4] {
        let mut list: [u8; 4] = Default::default();
        list.copy_from_slice(&(reg as u32).to_le_bytes());
        list
    }

    pub fn int_to_bytes(&self, int: u32) -> [u8; 4] {
        let mut list: [u8; 4] = Default::default();
        list.copy_from_slice(&int.to_le_bytes());
        list
    }

    pub fn memory_to_bytes(&self, mem: MemoryFetching) -> (u8, [u8; 4]) {
        match mem {
            MemoryFetching::Register(reg) => (REG, self.register_to_bytes(reg)),
            MemoryFetching::Addr(mem) => (MEM, self.int_to_bytes(mem as u32)),
        }
    }

    pub fn expr_to_bytes(&self, expr: Expr) -> (u8, [u8; 4]) {
        match expr {
            Expr::Int(int) => (INT, self.int_to_bytes(int as u32)),
            Expr::Label(label) => (LABEL, self.int_to_bytes(*self.labels.get(&label).unwrap())),
            Expr::Memory(mem) => self.memory_to_bytes(mem),
        }
    }
}

fn add_zero(string: String, size: usize) -> String {
    let added_zeros = if size.clone() < string.len() {
        "".to_string()
    } else {
        "0".repeat(size - string.len())
    };
    format!("{}{}", added_zeros, string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_string() {
        let instr = MachineCodeInstruction::new(
            0,
            MOV,
            REG,
            [0x10, 0x32, 0x45, 0x0],
            INT,
            [0x1, 0x0, 0x0, 0x0],
        );
        assert_eq!(instr.binary_string(), "0000 0010001 0001 00010000 00110010 01000101 00000000 0010 00000001 00000000 00000000 00000000");
    }

    #[test]
    fn test_hex_string() {
        let instr = MachineCodeInstruction::new(
            0,
            MOV,
            REG,
            [0xC8, 0xB, 0xA2, 0xB3],
            INT,
            [0x1, 0x0, 0x0, 0x0],
        );
        assert_eq!(
            instr.hex_string(),
            "0000 0011 0001 00c8 000b 00a2 00b3 0002 0001 0000 0000 0000"
        );
    }

    #[test]
    fn test_fmt_bin() {
        let instrs = MachineCode::new(vec![
            MachineCodeInstruction::new(
                0,
                MOV,
                REG,
                [0x1, 0x0, 0x0, 0x0],
                INT,
                [0x2, 0x0, 0x0, 0x0],
            ),
            MachineCodeInstruction::new(
                0,
                ADD,
                REG,
                [0x3, 0x0, 0x0, 0x0],
                INT,
                [0x9, 0x0, 0x0, 0x0],
            ),
        ]);

        assert_eq!(format!("{:b}", instrs), "0000 0010001 0001 00000001 00000000 00000000 00000000 0010 00000010 00000000 00000000 00000000\n0000 0010010 0001 00000011 00000000 00000000 00000000 0010 00001001 00000000 00000000 00000000\n");
    }

    #[test]
    fn test_fmt_hex() {
        let instrs = MachineCode::new(vec![
            MachineCodeInstruction::new(
                0,
                MOV,
                REG,
                [0x1, 0x0, 0x0, 0x0],
                INT,
                [0x2, 0x0, 0x0, 0x0],
            ),
            MachineCodeInstruction::new(
                0,
                MOV,
                REG,
                [0x3, 0x0, 0x0, 0x0],
                INT,
                [0x9, 0x0, 0x0, 0x0],
            ),
        ]);

        assert_eq!(format!("{:x}", instrs), "0000 0011 0001 0001 0000 0000 0000 0002 0002 0000 0000 0000\n0000 0011 0001 0003 0000 0000 0000 0002 0009 0000 0000 0000\n");
    }

    #[test]
    fn test_to_bytecode() {
        let instr = MachineCodeInstruction::new(
            0,
            MOV,
            REG,
            [0x1, 0x0, 0x0, 0x0],
            INT,
            [0x1, 0x0, 0x0, 0x0],
        );

        assert_eq!(
            instr.to_bytecode(),
            vec![0x0, MOV, REG, 0x1, 0x0, 0x0, 0x0, INT, 0x1, 0x0, 0x0, 0x0]
        );
    }

    #[test]
    fn test_mov() {
        let mut compiler = MachineCodeCompiler::new(Program::new(vec![Command::Mov(Mov(
            MemoryFetching::Register(Register::R1),
            Expr::Int(1),
        ))]));
        let machine_code = compiler.compile();
        assert_eq!(
            machine_code.code,
            vec![MachineCodeInstruction::new(
                0,
                MOV,
                REG,
                [0x1, 0x0, 0x0, 0x0],
                INT,
                [0x1, 0x0, 0x0, 0x0]
            )]
        );
    }

    #[test]
    fn test_pop() {
        let mut compiler = MachineCodeCompiler::new(Program::new(vec![Command::Pop(Pop(
            MemoryFetching::Register(Register::R1),
        ))]));
        let machine_code = compiler.compile();
        assert_eq!(
            machine_code.code,
            vec![MachineCodeInstruction::new(
                0,
                POP,
                REG,
                [0x1, 0x0, 0x0, 0x0],
                VOID,
                Default::default()
            )]
        );
    }
}
