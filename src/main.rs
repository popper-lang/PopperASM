use std::env::args;
use popper_asm::lexer::Lexer;
use popper_asm::parser::Parser;
use popper_asm::machine_code::{MachineCodeCompiler, MachineCodeInstruction};

fn main() {
    let args = args();
    let source = r#"
            main:
                mov r1, 3
                mov r2, 4
                call $sum

            sum:
                add r1, r2
        "#;

    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();

    let tokens = lexer.get_tokens();

    let mut parser = Parser::new(tokens);

    let program = parser.parse();

    match program {
        Ok(res) => {
            let mut c = MachineCodeCompiler::new(res);

            let m = c.compile();
            println!("Hex:\n{:x}", m);
            println!("Bin:\n{:b}", m);
        }
        Err(err) => {
            err.report(source);
        }
    }
}