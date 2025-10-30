use crate::ast::{
    AbstractSyntaxTree, AssignmentStatement, Expr, LetStatement, LoopStatement, PrintStatement,
    Statement, StatementList, Term,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum SemanticError {
    UndeclaredVariable(String),
}

pub struct ScopeStack {
    scopes: Vec<HashSet<String>>,
}

impl ScopeStack {
    pub fn new() -> Self {
        ScopeStack {
            scopes: vec![HashSet::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name);
        }
    }

    pub fn declared(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains(name) {
                return true;
            }
        }
        false
    }
}

pub struct SemanticAnalyzer {
    scope_stack: ScopeStack,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    fn new() -> Self {
        SemanticAnalyzer {
            scope_stack: ScopeStack::new(),
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
        self.scope_stack.declare(let_stmt.identifier.clone());
        self.analyze_expression(&let_stmt.value);
    }

    fn analyze_assignment_statement(&mut self, assign_stmt: &AssignmentStatement) {
        if !self.scope_stack.declared(&assign_stmt.identifier) {
            self.errors.push(SemanticError::UndeclaredVariable(
                assign_stmt.identifier.clone(),
            ));
        }
        self.analyze_expression(&assign_stmt.value);
    }

    fn analyze_loop_statement(&mut self, loop_stmt: &LoopStatement) {
        self.analyze_expression(&loop_stmt.count);
        self.scope_stack.enter_scope();
        self.analyze_statement_list(&loop_stmt.body.statements);
        self.scope_stack.exit_scope();
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
                if !self.scope_stack.declared(name) {
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

    #[test]
    fn test_loop_scope_violation() {
        let input = "loop 5 { let x = 10; }; print x;";
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
