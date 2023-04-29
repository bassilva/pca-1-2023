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

pub enum Value {
    Bool(bool),
    Var(String),
    Num(i32),
}

pub struct A1OpaA2 {
    pub a1: Value,
    pub a2: Value,
    pub opa: Opa,
}

pub struct B1OpbB2 {
    pub b1: Value,
    pub b2: Value,
    pub opb: Opb,
}

pub struct A1OprA2 {
    pub a1: Value,
    pub a2: Value,
    pub opr: Opr,
}

pub enum AExp {
    Var(Value),
    Num(Value),
    AritheticOp(A1OpaA2),
}

pub enum BExp {
    Boolean,
    NotBoolean,
    BooleanOp(B1OpbB2),
    RelationalOp(A1OprA2),
}

pub struct While {
    pub cond: BExp,
    pub statements: WhileProgram,
}

pub enum Stmt {
    Arithm(AExp),
    Boolean(BExp),
    Skip,
    While(While),
    // S1s2(S1s2),
    // IfElse(IfElse),
    // While(While),
}

pub struct Statement {
    pub lValue: Stmt,
    pub rValue: Option<Stmt>,
}

pub struct WhilelangType {
    pub statement: Statement,
    pub label: i32,
}

pub type WhileProgram = Vec<WhilelangType>;