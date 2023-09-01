use crate::ast::*;
use crate::lexer::{Token, TokenKind};
use crate::span::Span;

#[derive(Clone, Debug)]
pub struct Error {
    pub message: String,
    pub span: Span,
}

impl Error {
    pub fn new(message: String, span: Span) -> Self {
        Self { message, span }
    }

    pub fn report(&self, source: &str) {
        let extract = self.span.extract_from_str(source);
        let marker = self.span.make_marker(source);
        let line = source.lines().nth(self.span.find_line(source) - 1).unwrap();

        println!("Error:[{:?} {} `{:?}`", self.span, self.message, extract);
        println!("Source: {}", line);
        println!("{}", marker);
    }
}

#[derive(Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            errors: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<Program, Error> {
        let mut cmds = vec![];

        while !self.is_at_end() {
            let cmd = self.parse_label()?;
            cmds.push(cmd);
        }

        Ok(Program::new(cmds))
    }

    fn parse_command(&mut self) -> Result<Command, Error> {
        let command = self.expect(TokenKind::Ident)?;
        let command = match command.lexeme.as_str() {
            "mov" => self.parse_mov(),
            "add" => self.parse_add(),
            "sub" => self.parse_sub(),
            "mul" => self.parse_mul(),
            "div" => self.parse_div(),
            "pop" => self.parse_pop(),
            "call" => self.parse_call(),
            "allow" => self.parse_allow(),
            _ => Err(Error::new("unexpected command".to_string(), command.span)),
        };
        command
    }

    fn parse_mov(&mut self) -> Result<Command, Error> {
        let mem = self.parse_memory_fetching()?;
        let _ = self.expect(TokenKind::Comma);
        let expr = self.parse_expr()?;
        Ok(Command::Mov(Mov(mem, expr)))
    }

    fn parse_add(&mut self) -> Result<Command, Error> {
        let register = self.parse_memory_fetching()?;
        let _ = self.expect(TokenKind::Comma)?;
        let expr = self.parse_expr()?;

        Ok(Command::Add(Add(register, expr)))
    }

    fn parse_sub(&mut self) -> Result<Command, Error> {
        let register = self.parse_memory_fetching()?;
        let _ = self.expect(TokenKind::Comma)?;
        let expr = self.parse_expr()?;
        Ok(Command::Sub(Sub(register, expr)))
    }

    fn parse_mul(&mut self) -> Result<Command, Error> {
        let register = self.parse_memory_fetching()?;
        Ok(Command::Mul(Mul(register)))
    }

    fn parse_div(&mut self) -> Result<Command, Error> {
        let register = self.parse_memory_fetching()?;
        let _ = self.expect(TokenKind::Comma)?;
        let expr = self.parse_expr()?;
        Ok(Command::Div(Div(register, expr)))
    }

    fn parse_pop(&mut self) -> Result<Command, Error> {
        let register = self.parse_memory_fetching()?;
        Ok(Command::Pop(Pop(register)))
    }

    fn parse_call(&mut self) -> Result<Command, Error> {
        let label = self.parse_label_name()?;
        if let Expr::Label(label) = label {
            return Ok(Command::Call(Call(label)));
        }
        unreachable!()
    }

    fn parse_allow(&mut self) -> Result<Command, Error> {
        let to = self.parse_expr()?;
        let from = self.parse_expr()?;
        Ok(Command::Allow(Allow(to, from)))
    }

    fn parse_register(&mut self) -> Result<Register, Error> {
        let register = self.advance();
        let register = match register.token_kind {
            TokenKind::Ident => register,
            _ => {
                return Err(Error::new(
                    format!("Expected ident, found {:?}", register.token_kind),
                    register.span,
                ))
            }
        };
        let register = match register.lexeme.as_str() {
            "r1" => Register::R1,
            "r2" => Register::R2,
            "r3" => Register::R3,
            "r4" => Register::R4,
            "r5" => Register::R5,
            "r6" => Register::R6,
            "r7" => Register::R7,
            "r8" => Register::R8,
            "r9" => Register::R9,
            "r10" => Register::R10,
            "r11" => Register::R11,
            "r12" => Register::R12,
            "r13" => Register::R13,
            "r14" => Register::R14,
            "r15" => Register::R15,
            "rbp" => Register::Rbp,
            _ => {
                return Err(Error::new(
                    format!("Expected register, found {:?}", register.lexeme),
                    register.span,
                ))
            }
        };

        Ok(register)
    }

    fn parse_memory_fetching(&mut self) -> Result<MemoryFetching, Error> {
        match self.peek().token_kind {
            TokenKind::Ident => Ok(MemoryFetching::Register(self.parse_register()?)),
            TokenKind::Hashtag => Ok(MemoryFetching::Addr(self.parse_addr()?)),
            e => Err(Error::new(
                format!("Expected ident or hashtag, found {:?}", e),
                self.peek().span,
            )),
        }
    }

    fn parse_label(&mut self) -> Result<Label, Error> {
        let ident = self.expect(TokenKind::Ident)?;
        let _ = self.expect(TokenKind::Colon);
        let mut instrs = vec![];

        while !self.is_at_end() {
            self.ignore_newlines();
            let command = self.parse_command()?;
            instrs.push(command);
            let mut cloned_parser = self.clone();

            if cloned_parser.parse_label().is_ok() {
                break;
            }
        }

        Ok(Label::new(ident.lexeme, instrs))
    }

    fn parse_int(&mut self) -> Result<Expr, Error> {
        let int = self.peek();
        let int = match int.token_kind {
            TokenKind::Int => int,
            _ => {
                return Err(Error::new(
                    format!("Expected int, found {:?}", int.token_kind),
                    int.span,
                ))
            }
        };
        let int = match int.lexeme.parse::<i32>() {
            Ok(int) => int,
            Err(_) => {
                return Err(Error::new(
                    format!("Expected int, found {:?}", int.lexeme),
                    int.span,
                ))
            }
        };
        Ok(Expr::Int(int))
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        let expr = self.advance();
        let expr = match expr.token_kind {
            TokenKind::Int => self.parse_int()?,
            TokenKind::Dollar => self.parse_label_name()?,
            TokenKind::Hashtag | TokenKind::Ident => Expr::Memory(self.parse_memory_fetching()?),
            _ => {
                return Err(Error::new(
                    format!(
                        "Expected int, ident or hashtag, found {:?}",
                        expr.token_kind
                    ),
                    expr.span,
                ))
            }
        };
        Ok(expr)
    }

    fn parse_addr(&mut self) -> Result<usize, Error> {
        self.advance();
        let n = self
            .expect(TokenKind::Int)?
            .lexeme
            .parse::<usize>()
            .unwrap();
        Ok(n)
    }

    fn parse_label_name(&mut self) -> Result<Expr, Error> {
        let _ = self.expect(TokenKind::Dollar)?;
        let label_name = self.expect(TokenKind::Ident)?;
        Ok(Expr::Label(label_name.lexeme))
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.peek()
    }

    fn peek(&self) -> Token {
        if self.current == 0 {
            return self.tokens[self.current].clone();
        }
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn expect(&mut self, token_kind: TokenKind) -> Result<Token, Error> {
        let token = self.advance();
        if token.token_kind == token_kind {
            Ok(token)
        } else {
            Err(Error::new(
                format!(
                    "Expected {:?}, found {:?} `{}`",
                    token_kind, token.token_kind, token.lexeme
                ),
                token.span,
            ))
        }
    }

    fn ignore_newlines(&mut self) {
        while self.peek().token_kind == TokenKind::Newline {
            let _ = self.advance();
        }
    }
}
