pub enum Opa {
    Sum,
    Min,
    Mul,
    Div,
}

pub enum Opb {
    Not,
    And,
    Or,
}

pub enum Opr {
    Eq,
    Ne,
    Lt,
    Gt,
    Ge,
    Le,
}

pub struct A1OpaA2 {
    a1: i32,
    a2: i32,
    opa: Opa,
}

pub struct B1OpbA2 {
    b1: bool,
    ab: i32,
    opb: Opb,
}

pub struct B1OprA2 {
    b1: bool,
    ab: i32,
    opb: Opb,
}

pub enum Value {
    Bool(bool),
    Var(String),
    Num(i32),
}

pub struct Var {
    pub name: String,
    pub value: Value,
}

pub enum AExp {
    Var(Var),
    Num(Var),
    AritheticOp(A1OpaA2),
}

pub enum BExp {
    Boolean,
    NotBoolean,
    BooleanOp(B1OpbA2),
    RelationalOp(B1OprA2),
}

pub struct While {
    pub cond: BExp,
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Arithm(AExp),
    Boolean(BExp),
    Skip,
    While(While),
    // S1s2(S1s2),
    // IfElse(IfElse),
    // While(While),
}

pub struct WhilelangType {
    pub statement: Statement,
    pub label: Option<i32>,
}
