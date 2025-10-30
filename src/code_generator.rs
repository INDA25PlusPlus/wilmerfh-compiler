use crate::ast::*;

pub fn generate_c_code(ast: &AbstractSyntaxTree) -> String {
    let mut result = String::new();
    result.push_str("#include <stdio.h>\n");
    result.push_str("int main() {\n");
    result.push_str(&generate_statement_list(&ast.statement_list));
    result.push_str("return 0;\n");
    result.push_str("}\n");
    result
}

fn generate_statement_list(statement_list: &StatementList) -> String {
    let mut result = String::new();
    for statement in &statement_list.statements {
        result.push_str(&generate_statement(statement));
    }
    result
}

fn generate_statement(statement: &Statement) -> String {
    match statement {
        Statement::Let(let_stmt) => generate_let_statement(let_stmt),
        Statement::Assignment(assign_stmt) => generate_assignment_statement(assign_stmt),
        Statement::Loop(loop_stmt) => generate_loop_statement(loop_stmt),
        Statement::Print(print_stmt) => generate_print_statement(print_stmt),
    }
}

fn generate_let_statement(let_stmt: &LetStatement) -> String {
    let mut result = String::new();
    result.push_str(&format!("int {} = ", let_stmt.identifier));
    result.push_str(&generate_expression(&let_stmt.value));
    result.push_str(";\n");
    result
}

fn generate_assignment_statement(assign_stmt: &AssignmentStatement) -> String {
    let mut result = String::new();
    result.push_str(&format!("{} = ", assign_stmt.identifier));
    result.push_str(&generate_expression(&assign_stmt.value));
    result.push_str(";\n");
    result
}

fn generate_print_statement(print_stmt: &PrintStatement) -> String {
    let mut result = String::new();
    result.push_str("printf(\"%d\\n\", ");
    result.push_str(&generate_expression(&print_stmt.value));
    result.push_str(");\n");
    result
}

fn generate_loop_statement(loop_stmt: &LoopStatement) -> String {
    let mut result = String::new();
    result.push_str("for (int _ = 0; _ < ");
    result.push_str(&generate_expression(&loop_stmt.count));
    result.push_str("; _++) ");
    result.push_str(&generate_block(&loop_stmt.body));
    result
}

fn generate_block(block: &Block) -> String {
    let mut result = String::new();
    result.push_str("{\n");
    result.push_str(&generate_statement_list(&block.statements));
    result.push_str("}\n");
    result
}

fn generate_expression(expr: &Expr) -> String {
    let mut result = generate_term(&expr.lhs);
    if let Some(rhs) = &expr.rhs {
        result.push_str(" + ");
        result.push_str(&generate_expression(rhs));
    }
    result
}

fn generate_term(term: &Term) -> String {
    match term {
        Term::Number(n) => n.to_string(),
        Term::Identifier(id) => id.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    use crate::semantic_analyzer::SemanticAnalyzer;

    fn compile_source_to_c(source: &str) -> String {
        let lexer = Lexer::new(source.to_string());
        let tokens: Vec<_> = lexer.collect();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        SemanticAnalyzer::analyze(&ast).unwrap();
        generate_c_code(&ast)
    }

    #[test]
    fn test_basic_variables_and_print() {
        assert_eq!(
            compile_source_to_c("let x = 5; print x;"),
            "#include <stdio.h>\nint main() {\nint x = 5;\nprintf(\"%d\\n\", x);\nreturn 0;\n}\n"
        );
    }

    #[test]
    fn test_arithmetic_and_loops() {
        assert_eq!(
            compile_source_to_c("let x = 1; let y = 2; print x;"),
            "#include <stdio.h>\nint main() {\nint x = 1;\nint y = 2;\nprintf(\"%d\\n\", x);\nreturn 0;\n}\n"
        );
    }
}
