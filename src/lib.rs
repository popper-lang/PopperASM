#![allow(non_snake_case)]

pub mod lexer;
pub mod span;
pub mod parser;
pub mod ast;
pub mod machine_code;

#[derive(PartialEq, Debug)]
pub enum Mode { Debug, Release }

impl Mode {
    pub fn is_debug(&self) -> bool {
        self == &Mode::Debug
    }

    pub fn is_release(&self) -> bool {
        self == &Mode::Release
    }

}
pub static MODE: Mode = Mode::Release;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
