use crate::ast::{
    AbstractSyntaxTree, AssignmentStatement, Block, Expr, LetStatement, LoopStatement,
    PrintStatement, Statement, StatementList, Term,
};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume_token(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }

    pub fn parse(&mut self) -> AbstractSyntaxTree {
        let statements = self.parse_statement_list();
        AbstractSyntaxTree {
            statement_list: statements,
        }
    }

    fn parse_statement_list(&mut self) -> StatementList {
        let mut statements = Vec::new();
        while self.current_token().is_some() {
            statements.push(self.parse_statement());
        }
        StatementList { statements }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token() {
            Some(Token::Let) => Statement::Let(self.parse_let_statement()),
            Some(Token::Identifier(_)) => Statement::Assignment(self.parse_assignment_statement()),
            Some(Token::Loop) => Statement::Loop(self.parse_loop_statement()),
            Some(Token::Print) => Statement::Print(self.parse_print_statement()),
            _ => panic!("Unexpected token"),
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        let Some(Token::Let) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let Some(Token::Identifier(identifier)) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let Some(Token::Equals) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let value = self.parse_expression();
        let Some(Token::Semicolon) = self.consume_token() else {
            panic!("Unexpected token");
        };
        LetStatement { identifier, value }
    }

    fn parse_assignment_statement(&mut self) -> AssignmentStatement {
        let Some(Token::Identifier(identifier)) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let Some(Token::Equals) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let value = self.parse_expression();
        let Some(Token::Semicolon) = self.consume_token() else {
            panic!("Unexpected token");
        };
        AssignmentStatement { identifier, value }
    }

    fn parse_block(&mut self) -> Block {
        let Some(Token::OpenBracket) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let mut statements = Vec::new();
        while !matches!(self.current_token(), Some(Token::CloseBracket)) {
            statements.push(self.parse_statement());
        }
        let Some(Token::CloseBracket) = self.consume_token() else {
            panic!("Unexpected token");
        };
        Block {
            statements: Box::new(StatementList { statements }),
        }
    }

    fn parse_loop_statement(&mut self) -> LoopStatement {
        let Some(Token::Loop) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let condition = self.parse_expression();
        let body = self.parse_block();
        let Some(Token::Semicolon) = self.consume_token() else {
            panic!("Unexpected token");
        };
        LoopStatement {
            count: condition,
            body: Box::new(body),
        }
    }

    fn parse_print_statement(&mut self) -> PrintStatement {
        let Some(Token::Print) = self.consume_token() else {
            panic!("Unexpected token");
        };
        let value = self.parse_expression();
        let Some(Token::Semicolon) = self.consume_token() else {
            panic!("Unexpected token");
        };
        PrintStatement { value }
    }

    fn parse_expression(&mut self) -> Expr {
        let lhs = match self.consume_token() {
            Some(Token::Identifier(name)) => Term::Identifier(name.clone()),
            Some(Token::Number(n)) => Term::Number(n),
            _ => panic!("Unexpected token"),
        };
        let rhs = if matches!(self.current_token(), Some(Token::Plus)) {
            _ = self.consume_token();
            Some(Box::new(self.parse_expression()))
        } else {
            None
        };

        Expr { lhs, rhs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_program() {
        // let x = 5; loop 3 { x = x + 1; print x; }
        let tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Equals,
            Token::Number(5),
            Token::Semicolon,
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

        let mut parser = Parser::new(tokens);
        parser.parse();
    }

    #[test]
    #[should_panic]
    fn test_invalid_program() {
        // missing semicolon after let statement
        // let a = 1 + 2 print a;
        let tokens = vec![
            Token::Let,
            Token::Identifier("a".to_string()),
            Token::Equals,
            Token::Number(1),
            Token::Plus,
            Token::Number(2),
            // Missing Semicolon here
            Token::Print,
            Token::Identifier("a".to_string()),
            Token::Semicolon,
        ];

        let mut parser = Parser::new(tokens);
        parser.parse();
    }
}
