#![allow(dead_code)]

pub enum Term {
    Identifier(String),
    Number(i32),
}

pub struct Expr {
    pub lhs: Term,
    pub rhs: Option<Box<Expr>>,
}

pub struct LetStatement {
    pub identifier: String,
    pub value: Expr,
}

pub struct AssignmentStatement {
    pub identifier: String,
    pub value: Expr,
}

pub struct Block {
    pub statements: Box<StatementList>,
}

pub struct LoopStatement {
    pub count: Expr,
    pub body: Box<Block>,
}

pub struct PrintStatement {
    pub value: Expr,
}

pub enum Statement {
    Let(LetStatement),
    Assignment(AssignmentStatement),
    Loop(LoopStatement),
    Print(PrintStatement),
}

pub struct StatementList {
    pub statements: Vec<Statement>,
}

pub struct AbstractSyntaxTree {
    pub statements: StatementList,
}
