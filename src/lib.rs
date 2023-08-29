#![allow(non_snake_case)]

pub mod lexer;
pub mod span;
pub mod parser;
pub mod ast;
pub mod machine_code;

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
