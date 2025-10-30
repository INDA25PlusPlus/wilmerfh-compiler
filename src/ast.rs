#![allow(dead_code)]

#[derive(Debug)]
pub enum Term {
    Identifier(String),
    Number(i32),
}

#[derive(Debug)]
pub struct Expr {
    pub lhs: Term,
    pub rhs: Option<Box<Expr>>,
}

#[derive(Debug)]
pub struct LetStatement {
    pub identifier: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct AssignmentStatement {
    pub identifier: String,
    pub value: Expr,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Box<StatementList>,
}

#[derive(Debug)]
pub struct LoopStatement {
    pub count: Expr,
    pub body: Box<Block>,
}

#[derive(Debug)]
pub struct PrintStatement {
    pub value: Expr,
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Assignment(AssignmentStatement),
    Loop(LoopStatement),
    Print(PrintStatement),
}

#[derive(Debug)]
pub struct StatementList {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub statements: StatementList,
}
