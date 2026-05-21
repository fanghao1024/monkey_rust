use crate::token::{self, Token, TokenType};

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.as_bytes().to_vec(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skipWhitespace();
        println!("ch:{:?}", self.ch as char);
        let tok = match self.ch {
            b'=' => new_token(TokenType::ASSIGN, self.ch),
            b';' => new_token(TokenType::SEMICOLON, self.ch),
            b'(' => new_token(TokenType::LPARAM, self.ch),
            b')' => new_token(TokenType::RPARAM, self.ch),
            b',' => new_token(TokenType::COMMA, self.ch),
            b'+' => new_token(TokenType::PLUS, self.ch),
            b'{' => new_token(TokenType::LBRACE, self.ch),
            b'}' => new_token(TokenType::RBRACE, self.ch),
            0 => Token::new(TokenType::EOF, ""),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = token::lookup_ident(&literal);
                    return Token::new(token_type, literal);
                } else if is_digit(self.ch) {
                    return Token::new(TokenType::INT, self.read_number());
                } else {
                    new_token(TokenType::ILLEGAL, self.ch)
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).into_owned()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).into_owned()
    }

    fn skipWhitespace(&mut self) {
        while matches!(self.ch, b' ' | b'\t' | b'\n' | b'\r') {
            self.read_char();
        }
    }
}

fn new_token(token_type: TokenType, ch: u8) -> Token {
    Token::new(token_type, (ch as char).to_string())
}

fn is_letter(ch: u8) -> bool {
    (b'a' <= ch && ch <= b'z') || (b'A' <= ch && ch <= b'Z') || ch == b'_'
}

fn is_digit(ch: u8) -> bool {
    b'0' <= ch && ch <= b'9'
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
    let ten =10;

    let add=fn(x,y){
    x+y;
    };

    let result=add(five,ten);
    "#;
        let tests = vec![
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "5"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::INT, "10"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::FUNCTION, "fn"),
            Token::new(TokenType::LPARAM, "("),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::RPARAM, ")"),
            Token::new(TokenType::LBRACE, "{"),
            Token::new(TokenType::IDENT, "x"),
            Token::new(TokenType::PLUS, "+"),
            Token::new(TokenType::IDENT, "y"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::RBRACE, "}"),
            Token::new(TokenType::SEMICOLON, ";"),
            Token::new(TokenType::LET, "let"),
            Token::new(TokenType::IDENT, "result"),
            Token::new(TokenType::ASSIGN, "="),
            Token::new(TokenType::IDENT, "add"),
            Token::new(TokenType::LPARAM, "("),
            Token::new(TokenType::IDENT, "five"),
            Token::new(TokenType::COMMA, ","),
            Token::new(TokenType::IDENT, "ten"),
            Token::new(TokenType::RPARAM, ")"),
            Token::new(TokenType::SEMICOLON, ";"),
        ];
        let mut lexer = Lexer::new(input);
        for (i, expected) in tests.into_iter().enumerate() {
            let tok = lexer.next_token();

            println!("expected = {:?}, got = {:?}", expected.literal, tok.literal);

            assert_eq!(
                tok.token_type, expected.token_type,
                "test[{i}] - tokentype wrong. expected={:?}, got ={:?}",
                expected.token_type, tok.token_type
            );

            assert_eq!(
                tok.literal, expected.literal,
                "test[{i}] - literal wrong. expected={:?}, got = {:?}",
                expected.literal, tok.literal
            );
        }
    }
}
