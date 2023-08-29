use std::env::args;
use PopperASM::lexer::Lexer;
use PopperASM::parser::Parser;

fn main() {
    let args = args();
    let source = r#"
            main:
                mov r1, 3
                mov r2, 9
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
            println!("{:#?}", res)
        }
        Err(err) => {
            err.report(source);
        }
    }
}