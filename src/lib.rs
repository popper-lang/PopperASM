#![allow(non_snake_case)]

pub mod ast;
pub mod lexer;
pub mod machine_code;
pub mod parser;
pub mod span;

#[derive(PartialEq, Debug)]
pub enum Mode {
    Debug,
    Release,
}

impl Mode {
    pub fn is_debug(&self) -> bool {
        self == &Mode::Debug
    }

    pub fn is_release(&self) -> bool {
        self == &Mode::Release
    }
}

pub static MODE: Mode = Mode::Release;

pub fn compile_file_into_file(file_name: &str, output_file_name: &str) {
    let body = std::fs::read_to_string(file_name).unwrap();
    let binary = compile_string(body.as_str());
    std::fs::write(output_file_name, binary).unwrap();
}

pub fn compile_string(string: &str) -> String {
    let mut lexer = lexer::Lexer::new(string);
    lexer.scan_tokens();
    let mut parser = parser::Parser::new(lexer.get_tokens());
    let out = parser.parse().unwrap();
    let mut machine_code_compiler = machine_code::MachineCodeCompiler::new(out);
    let m = machine_code_compiler.compile();
    format!("{:b}", m)
}