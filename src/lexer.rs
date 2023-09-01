use crate::span::Span;

// token for the assembler
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TokenKind {
    Ident,
    String,
    Int,
    Colon,
    Hashtag,
    Comma,
    Dollar,
    Eof,
    Newline,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

#[derive(Clone)]
pub struct Lexer<'a> {
    pub source: &'a str,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ':' => self.add_token(TokenKind::Colon),
            '#' => self.add_token(TokenKind::Hashtag),
            ',' => self.add_token(TokenKind::Comma),
            '$' => self.add_token(TokenKind::Dollar),
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            '\n' => {
                self.line += 1;
            }
            ' ' | '\r' | '\t' => (),
            e => panic!("Unexpected character: {}", e),
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        self.add_token(TokenKind::Ident);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        self.add_token(TokenKind::Int);
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Unterminated string");
        }

        self.advance();

        let _lexeme = self.source[self.start + 1..self.current - 1].to_string();
        let token_kind = TokenKind::String;
        self.add_token(token_kind);
    }

    fn add_token(&mut self, token_kind: TokenKind) {
        let lexeme = self.source[self.start..self.current].to_string();
        let span = Span::new(self.start, self.current);
        self.tokens.push(Token {
            token_kind,
            lexeme,
            span,
        });
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident() {
        let mut lexer = Lexer::new("mov");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::Ident);
        assert_eq!(tokens[0].lexeme, "mov");
    }

    #[test]
    fn test_int() {
        let mut lexer = Lexer::new("123");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::Int);
        assert_eq!(tokens[0].lexeme, "123");
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new("\"hello\"");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::String);
        assert_eq!(tokens[0].lexeme, "\"hello\"");
    }

    #[test]
    fn test_colon() {
        let mut lexer = Lexer::new(":");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::Colon);
        assert_eq!(tokens[0].lexeme, ":");
    }

    #[test]
    fn test_hashtag() {
        let mut lexer = Lexer::new("#");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::Hashtag);
        assert_eq!(tokens[0].lexeme, "#");
    }

    #[test]
    fn test_comma() {
        let mut lexer = Lexer::new(",");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::Comma);
        assert_eq!(tokens[0].lexeme, ",");
    }

    #[test]
    fn test_all() {
        let mut lexer = Lexer::new("mov 123 : # , \"hello\"");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].token_kind, TokenKind::Ident);
        assert_eq!(tokens[0].lexeme, "mov");
        assert_eq!(tokens[1].token_kind, TokenKind::Int);
        assert_eq!(tokens[1].lexeme, "123");
        assert_eq!(tokens[2].token_kind, TokenKind::Colon);
        assert_eq!(tokens[2].lexeme, ":");
        assert_eq!(tokens[3].token_kind, TokenKind::Hashtag);
        assert_eq!(tokens[3].lexeme, "#");
        assert_eq!(tokens[4].token_kind, TokenKind::Comma);
        assert_eq!(tokens[4].lexeme, ",");
        assert_eq!(tokens[5].token_kind, TokenKind::String);
        assert_eq!(tokens[5].lexeme, "\"hello\"");
    }

    #[test]
    fn test_span() {
        let mut lexer = Lexer::new("mov 123 : # , \"hello\"");
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].span, Span::new(0, 3));
        assert_eq!(tokens[1].span, Span::new(4, 7));
        assert_eq!(tokens[2].span, Span::new(8, 9));
        assert_eq!(tokens[3].span, Span::new(10, 11));
        assert_eq!(tokens[4].span, Span::new(12, 13));
        assert_eq!(tokens[5].span, Span::new(14, 21));
    }

    #[test]
    fn test_extract_span() {
        let source = "mov 123 : # , \"hello\"";
        let mut lexer = Lexer::new(source);
        lexer.scan_tokens();
        let tokens = lexer.get_tokens();
        assert_eq!(tokens[0].span.extract_from_str(source), "mov");
        assert_eq!(tokens[1].span.extract_from_str(source), "123");
        assert_eq!(tokens[2].span.extract_from_str(source), ":");
        assert_eq!(tokens[3].span.extract_from_str(source), "#");
        assert_eq!(tokens[4].span.extract_from_str(source), ",");
        assert_eq!(tokens[5].span.extract_from_str(source), "\"hello\"");
    }
}
