#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Let,
    Loop,
    Plus,
    Equals,
    Semicolon,
    OpenBracket,
    CloseBracket,
    Print,
}

pub struct Lexer {
    src: String,
    pos: usize,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Lexer { src, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn current_char(&self) -> Option<char> {
        self.src.chars().nth(self.pos)
    }

    fn try_parse_identifier(&mut self) -> Option<Token> {
        let start = self.pos;
        let Some(c) = self.current_char() else {
            return None;
        };
        if !c.is_alphabetic() {
            return None;
        }
        self.pos += 1;
        while let Some(c) = self.current_char() {
            if !c.is_alphanumeric() {
                break;
            } else {
                self.pos += 1;
            }
        }
        let identifier = &self.src[start..self.pos];
        match identifier {
            "let" => Some(Token::Let),
            "loop" => Some(Token::Loop),
            "print" => Some(Token::Print),
            _ => Some(Token::Identifier(identifier.to_string())),
        }
    }

    fn try_parse_number(&mut self) -> Option<Token> {
        let start = self.pos;
        let Some(c) = self.current_char() else {
            return None;
        };
        if !c.is_digit(10) {
            return None;
        }
        self.pos += 1;
        while let Some(c) = self.current_char() {
            if !c.is_digit(10) {
                break;
            } else {
                self.pos += 1;
            }
        }
        let number_str = &self.src[start..self.pos];
        Some(Token::Number(number_str.parse::<i32>().unwrap()))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let current_char = self.current_char()?;
        if let Some(token) = self.try_parse_identifier() {
            return Some(token);
        }
        if let Some(token) = self.try_parse_number() {
            return Some(token);
        }
        if let Some(token) = match current_char {
            '+' => Some(Token::Plus),
            '=' => Some(Token::Equals),
            ';' => Some(Token::Semicolon),
            '{' => Some(Token::OpenBracket),
            '}' => Some(Token::CloseBracket),
            _ => None,
        } {
            self.pos += 1;
            return Some(token);
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_program() {
        let input = "let x = 5; loop 3 { x = x + 1; print x; };";

        let expected_tokens = vec![
            // let x = 5;
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Number(5),
            Token::Semicolon,
            // loop 3 { x = x + 1; print x; }
            Token::Loop,
            Token::Number(3),
            Token::OpenBracket,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Number(1),
            Token::Semicolon,
            Token::Print,
            Token::Identifier("x".to_string()),
            Token::Semicolon,
            Token::CloseBracket,
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input.to_string());
        let actual_tokens: Vec<Token> = lexer.collect();

        assert_eq!(actual_tokens, expected_tokens);
    }

    #[test]
    #[should_panic]
    fn test_stops_at_invalid_char() {
        let input = "let x = @123";
        let lexer = Lexer::new(input.to_string());
        let _: Vec<Token> = lexer.collect();
    }
}
