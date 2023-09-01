use popper_asm::lexer::Lexer;
use popper_asm::parser::Parser;
use popper_asm::machine_code::{MachineCodeCompiler, MachineCodeInstruction};
use popper_asm::MODE;

fn main() {
    if MODE.is_debug() {
        debug();
    } else if MODE.is_release() {
        cli();
    }
}

fn debug() {
    let source = r#"

    main:
        mov r1, 2
        mov r2, 3
        mov r4, 5
        call $sum
    sum:
        add r1, r4
    "#;

    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();
    let mut parser = Parser::new(lexer.get_tokens());
    match parser.parse() {
        Ok(out) => {
            let mut machine_code_compiler = MachineCodeCompiler::new(out);

            let m = machine_code_compiler.compile();
            println!("Binary:\n{:b}", m);
        },
        Err(e) => {
            e.report(source);
        }
    };
}

fn cli() {
    let args: Vec<String> = std::env::args().collect();
    let input = args.get(1).expect("Expected <input>");
    let output = args.get(2).expect("Expected <output>");
    let content_input = std::fs::read_to_string(input).expect("Dont find the file");

    let mut lexer = Lexer::new(&content_input);
    lexer.scan_tokens();
    let mut parser = Parser::new(lexer.get_tokens());

    match parser.parse() {
        Ok(out) => {
            let mut machine_code_compiler = MachineCodeCompiler::new(out);

            let m = machine_code_compiler.compile();

            std::fs::write(output, format!("{:b}", m)).expect("Unable to write file");
        },
        Err(e) => {
            e.report(&content_input);
        }
    };
}