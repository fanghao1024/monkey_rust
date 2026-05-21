use std::io::{BufRead, Write};

use crate::lexer;
use crate::token;

pub const PROMOT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut reader: R, mut writer: W) {
    let mut line = String::new();

    loop {
        writer.write_all(PROMOT.as_bytes()).unwrap();
        writer.flush().unwrap();

        line.clear();
        let bytes_read = reader.read_line(&mut line).unwrap();
        if bytes_read == 0 {
            return;
        }

        let line = line.strip_suffix("\n").unwrap_or(&line);
        let line = line.strip_suffix("\r").unwrap_or(line);

        let mut lexer = lexer::Lexer::new(line);
        loop {
            let tok = lexer.next_token();
            if tok.token_type == token::TokenType::EOF {
                break;
            }
            writeln!(writer, "{:?}", tok).unwrap();
        }
    }
}
