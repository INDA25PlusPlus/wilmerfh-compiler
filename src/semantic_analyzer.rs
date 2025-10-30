use crate::ast::{
    AbstractSyntaxTree, AssignmentStatement, Expr, LetStatement, LoopStatement, PrintStatement,
    Statement, StatementList, Term,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum SemanticError {
    UndeclaredVariable(String),
}

pub struct SymbolTable {
    declared_variables: HashSet<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            declared_variables: HashSet::new(),
        }
    }

    pub fn declare(&mut self, name: String) {
        self.declared_variables.insert(name);
    }

    pub fn declared(&self, name: &str) -> bool {
        self.declared_variables.contains(name)
    }
}

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    fn new() -> Self {
        SemanticAnalyzer {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(ast: &AbstractSyntaxTree) -> Result<(), Vec<SemanticError>> {
        let mut analyzer = SemanticAnalyzer::new();
        analyzer.analyze_statement_list(&ast.statement_list);

        if analyzer.errors.is_empty() {
            Ok(())
        } else {
            Err(analyzer.errors.clone())
        }
    }

    fn analyze_statement_list(&mut self, statement_list: &StatementList) {
        for statement in &statement_list.statements {
            self.analyze_statement(statement);
        }
    }

    fn analyze_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Let(let_stmt) => self.analyze_let_statement(let_stmt),
            Statement::Assignment(assign_stmt) => self.analyze_assignment_statement(assign_stmt),
            Statement::Loop(loop_stmt) => self.analyze_loop_statement(loop_stmt),
            Statement::Print(print_stmt) => self.analyze_print_statement(print_stmt),
        }
    }

    fn analyze_let_statement(&mut self, let_stmt: &LetStatement) {
        self.symbol_table.declare(let_stmt.identifier.clone());
        self.analyze_expression(&let_stmt.value);
    }

    fn analyze_assignment_statement(&mut self, assign_stmt: &AssignmentStatement) {
        if !self.symbol_table.declared(&assign_stmt.identifier) {
            self.errors.push(SemanticError::UndeclaredVariable(
                assign_stmt.identifier.clone(),
            ));
        }
        self.analyze_expression(&assign_stmt.value);
    }

    fn analyze_loop_statement(&mut self, loop_stmt: &LoopStatement) {
        self.analyze_expression(&loop_stmt.count);
        self.analyze_statement_list(&loop_stmt.body.statements);
    }

    fn analyze_print_statement(&mut self, print_stmt: &PrintStatement) {
        self.analyze_expression(&print_stmt.value);
    }

    fn analyze_expression(&mut self, expr: &Expr) {
        self.analyze_term(&expr.lhs);
        if let Some(rhs_expr) = &expr.rhs {
            self.analyze_expression(rhs_expr);
        }
    }

    fn analyze_term(&mut self, term: &Term) {
        match term {
            Term::Identifier(name) => {
                if !self.symbol_table.declared(name) {
                    self.errors
                        .push(SemanticError::UndeclaredVariable(name.clone()));
                }
            }
            Term::Number(_) => {
                // Numbers are always valid
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_valid_program() {
        let input = "let x = 5; print x;";
        let lexer = Lexer::new(input.to_string());
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let result = SemanticAnalyzer::analyze(&ast);

        assert!(result.is_ok());
    }

    #[test]
    fn test_undeclared_variable() {
        let input = "print x;";
        let lexer = Lexer::new(input.to_string());
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let result = SemanticAnalyzer::analyze(&ast);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            SemanticError::UndeclaredVariable(name) => assert_eq!(name, "x"),
        }
    }

    #[test]
    fn test_undeclared_assignment() {
        let input = "x = 5;";
        let lexer = Lexer::new(input.to_string());
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        let result = SemanticAnalyzer::analyze(&ast);

        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        match &errors[0] {
            SemanticError::UndeclaredVariable(name) => assert_eq!(name, "x"),
        }
    }
}
