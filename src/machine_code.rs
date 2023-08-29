use std::collections::HashMap;
use std::fmt::{Binary, LowerHex};
use crate::ast::*;

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


#[derive(Clone, Debug, PartialEq)]
pub struct MachineCodeInstruction {
    pub label: u8,
    pub instr: u8,
    pub operand1_type: u8,
    pub operand1: [u8; 4],
    pub operand2_type: u8,
    pub operand2: [u8; 4],
}

impl MachineCodeInstruction {
    pub fn new(label: u8, instr: u8, operand1_type: u8, operand1: [u8; 4], operand2_type: u8, operand2: [u8; 4]) -> Self {
        Self {
            label,
            instr,
            operand1_type,
            operand1,
            operand2_type,
            operand2
        }
    }

    pub fn binary_string(&self) -> String {
        let label_bin = add_zero(format!("{:b}", self.label), 8);
        let instr_bin = add_zero(format!("{:b}", self.instr), 7);
        let operand1_type_bin = add_zero(format!("{:b}", self.operand1_type), 4);
        let mut operand1_bin = String::new();
        for byte in &self.operand1 {
            operand1_bin.push_str(&add_zero(format!("{:b} ", byte), 5));
        }
        let operand2_type_bin = add_zero(format!("{:b}", self.operand2_type), 4);
        let mut operand2_bin = String::new();
        for byte in &self.operand2 {
            operand2_bin.push_str(&add_zero(format!("{:b} ", byte), 5));
        }
        format!("{} {} {} {} {} {}", label_bin, instr_bin, operand1_type_bin, operand1_bin, operand2_type_bin, operand2_bin)
    }

    pub fn hex_string(&self) -> String {
        let label_hex = add_zero(format!("{:x}", self.label), 5);
        let instr_hex = add_zero(format!("{:x}", self.instr), 5);
        let operand1_type_hex = add_zero(format!("{:x}", self.operand1_type), 5);
        let mut operand1_hex = String::new();
        for byte in &self.operand1 {
            operand1_hex.push_str(&add_zero(format!("{:x} ", byte), 5));
        }
        let operand2_type_hex = add_zero(format!("{:x}", self.operand2_type), 5);
        let mut operand2_hex = String::new();
        for byte in &self.operand2 {
            operand2_hex.push_str(&add_zero(format!("{:x} ", byte), 5));
        }
        format!("{} {} {} {} {} {}", label_hex, instr_hex, operand1_type_hex, operand1_hex, operand2_type_hex, operand2_hex)
    }
}

#[derive(Clone, Debug)]
pub struct MachineCode {
    code: Vec<MachineCodeInstruction>
}



impl MachineCode {
    pub fn new(code: Vec<MachineCodeInstruction>) -> Self {
        Self {
            code
        }
    }

    pub fn push(&mut self, instr: MachineCodeInstruction) {
        self.code.push(instr);
    }

    pub fn extend<I: IntoIterator<Item = MachineCodeInstruction>>(&mut self, instrs: I) {
        self.code.extend(instrs);
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
            string.push_str("\n");
        }

        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
pub struct MachineCodeCompiler {
    pub program: Program,
    pub machine_code: MachineCode,
    labels: HashMap<String, u8>,
    current_label: u8
}

impl MachineCodeCompiler {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            machine_code: MachineCode::new(vec![]),
            labels: HashMap::new(),
            current_label: 0
        }
    }

    pub fn compile(&mut self) -> MachineCode {
        self.command_compiler(self.program.commands.clone());
        self.machine_code.clone()
    }

    pub fn command_compiler(&mut self, commands: Vec<Command>)  {
        for cmd in commands.clone() {
            match cmd {
                Command::Mov(mov) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(mov.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(mov.1);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, MOV, operand1_type, operand1, operand2_type, operand2));
                },
                Command::Add(add) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(add.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(add.1);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, ADD, operand1_type, operand1, operand2_type, operand2));
                },
                Command::Sub(sub) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(sub.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(sub.1);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, SUB, operand1_type, operand1, operand2_type, operand2));
                },
                Command::Mul(mul) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(mul.0);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, MUL, operand1_type, operand1, VOID, Default::default()));
                },
                Command::Div(div) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(div.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(div.1);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, DIV, operand1_type, operand1, operand2_type, operand2));
                },
                Command::Pop(pop) => {
                    let (operand1_type, operand1) = self.memory_to_bytes(pop.0);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, POP, operand1_type, operand1, VOID, Default::default()));
                },
                Command::Call(call) => {
                    let operand1_type = INT;
                    let operand1 = self.int_to_bytes(*self.labels.get(&call.0).unwrap() as i32);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, CALL, operand1_type, operand1, VOID, Default::default()));
                },
                Command::Allow(allow) => {
                    let (operand1_type, operand1) = self.expr_to_bytes(allow.0);
                    let (operand2_type, operand2) = self.expr_to_bytes(allow.1);

                    self.machine_code.push(MachineCodeInstruction::new(self.current_label, ALLOW, operand1_type, operand1, operand2_type, operand2));
                },
                Command::Label(label) => {
                    let last = self.labels.iter().last().map(|x| x.1).unwrap_or(&0);
                    let id = last + 1;
                    self.labels.insert(label.name, id.clone());
                    self.current_label = id;
                    self.command_compiler(label.program.commands);
                }
            }
        }
    }

    pub fn register_to_bytes(&self, reg: Register) -> [u8; 4] {
        let mut list: [u8; 4] = Default::default();
        list.copy_from_slice(&(reg as u32).to_le_bytes());
        list
    }

    pub fn int_to_bytes(&self, int: i32) -> [u8; 4] {
        let mut list: [u8; 4] = Default::default();
        list.copy_from_slice(&int.to_le_bytes());
        list
    }

    pub fn memory_to_bytes(&self, mem: MemoryFetching) -> (u8, [u8; 4]) {
        match mem {
            MemoryFetching::Register(reg) => (REG, self.register_to_bytes(reg)),
            MemoryFetching::Addr(mem) => (MEM, self.int_to_bytes(mem as i32)),
        }
    }

    pub fn expr_to_bytes(&self, expr: Expr) -> (u8, [u8; 4])  {
        match expr {
            Expr::Int(int) => (INT, self.int_to_bytes(int)),
            Expr::Label(label) => (LABEL, self.int_to_bytes(self.labels.get(&label).unwrap().clone() as i32)),
            Expr::Memory(mem) => self.memory_to_bytes(mem),
        }
    }


}

fn add_zero(string: String, size: usize) -> String {
    let added_zeros = "0".repeat(size - string.len());
    format!("{}{}", added_zeros, string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_string() {
        let instr = MachineCodeInstruction::new(0, MOV, REG, [0x1, 0x0, 0x0, 0x0], INT, [0x1, 0x0, 0x0, 0x0]);
        assert_eq!(instr.binary_string(), "00000000 0010001 0001 0001 0000 0000 0000  0010 0001 0000 0000 0000 ");
    }

    #[test]
    fn test_hex_string() {
        let instr = MachineCodeInstruction::new(0, MOV, REG, [0xA, 0x0, 0x0, 0x0], INT, [0x1, 0x0, 0x0, 0x0]);
        assert_eq!(instr.hex_string().trim(), "00 11 0001 000a 0000 0000 0000  0002 0001 0000 0000 0000");
    }

    #[test]
    fn test_fmt_bin() {
        let instrs = MachineCode::new(
            vec![
                MachineCodeInstruction::new(0, MOV, REG, [0x1, 0x0, 0x0, 0x0], INT, [0x2, 0x0, 0x0, 0x0]),
                MachineCodeInstruction::new(0, ADD, REG, [0x3, 0x0, 0x0, 0x0], INT, [0x9, 0x0, 0x0, 0x0]),
            ]
        );

        assert_eq!(format!("{:b}", instrs), "00000000 0010001 0001 0001 0000 0000 0000  0010 0010 0000 0000 0000 \n00000000 0010010 0001 0011 0000 0000 0000  0010 1001 0000 0000 0000 \n");
    }

    #[test]
    fn test_fmt_hex() {
        let instrs = MachineCode::new(
            vec![
                MachineCodeInstruction::new(0, MOV, REG, [0x1, 0x0, 0x0, 0x0], INT, [0x2, 0x0, 0x0, 0x0]),
                MachineCodeInstruction::new(0, MOV, REG, [0x3, 0x0, 0x0, 0x0], INT, [0x9, 0x0, 0x0, 0x0]),
            ]
        );

        assert_eq!(format!("{:x}", instrs), "00 11 0001 0001 0000 0000 0000  0002 0002 0000 0000 0000 \n00 11 0001 0003 0000 0000 0000  0002 0009 0000 0000 0000 \n");
    }
    #[test]
    fn test_mov() {
        let mut compiler = MachineCodeCompiler::new(Program::new(vec![Command::Mov(Mov(MemoryFetching::Register(Register::R1), Expr::Int(1)))]));
        let machine_code = compiler.compile();
        assert_eq!(machine_code.code, vec![
            MachineCodeInstruction::new(0, MOV, REG, [0x1, 0x0, 0x0, 0x0], INT, [0x1, 0x0, 0x0, 0x0])
        ]);
    }

    #[test]
    fn test_pop() {
        let mut compiler = MachineCodeCompiler::new(Program::new(vec![Command::Pop(Pop(MemoryFetching::Register(Register::R1)))]));
        let machine_code = compiler.compile();
        assert_eq!(machine_code.code, vec![
            MachineCodeInstruction::new(0, POP, REG, [0x1, 0x0, 0x0, 0x0], VOID, Default::default())
        ]);
    }


}
