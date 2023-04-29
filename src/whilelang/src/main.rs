mod types;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "while.pest"]
pub struct WhileParser;

fn main() {
    let successful_parse_num = WhileParser::parse(Rule::Num, "1");
    let successful_parse_var = WhileParser::parse(Rule::Var, "x1");
    let successful_parse_lab = WhileParser::parse(Rule::Lab, "l");

    let successful_parse_aexp1 = WhileParser::parse(Rule::AExp, "x");
    let successful_parse_aexp2 = WhileParser::parse(Rule::AExp, "n");
    let successful_parse_aexp3 = WhileParser::parse(Rule::AExp, "a1 + a2");

    let successful_parse_bexp1 = WhileParser::parse(Rule::BExp, "true");
    let successful_parse_bexp2 = WhileParser::parse(Rule::BExp, "false");
    let successful_parse_bexp3 = WhileParser::parse(Rule::BExp, "not b");

    let successful_parse_stmt1 = WhileParser::parse(Rule::assignment, "x := 9");
    let successful_parse_stmt2 = WhileParser::parse(Rule::assignment, "x := read");
    let successful_parse_stmt3 = WhileParser::parse(Rule::assignment, "x := a + b");

    let successful_parse_stmt4 = WhileParser::parse(
        Rule::conditional,
        "if x < a then { a := 4 } else { a := b }",
    );
    let successful_parse_stmt5 = WhileParser::parse(Rule::conditional, "if x < a then { a := 4 }");
    let successful_parse_stmt6 =
        WhileParser::parse(Rule::loopwhile, "while x !== a do { a := a - 1 }");

    let successful_parse_stmt7 = WhileParser::parse(Rule::print, "print \"hello world\"");
    let successful_parse_stmt8 = WhileParser::parse(Rule::print, "print x + a");
    let successful_parse_stmt9 = WhileParser::parse(Rule::write, "write x");
    let successful_parse_stmt10 = WhileParser::parse(Rule::read, "read");
    let successful_parse_stmt11 = WhileParser::parse(Rule::skip, "skip");

    println!("{:?}", successful_parse_num);
    println!("{:?}", successful_parse_var);
    println!("{:?}", successful_parse_lab);

    println!("{:?}", successful_parse_aexp1);
    println!("{:?}", successful_parse_aexp2);
    println!("{:?}", successful_parse_aexp3);

    println!("{:?}", successful_parse_bexp1);
    println!("{:?}", successful_parse_bexp2);
    println!("{:?}", successful_parse_bexp3);

    println!("{:?}", successful_parse_stmt1);
    println!("{:?}", successful_parse_stmt2);
    println!("{:?}", successful_parse_stmt3);

    println!("{:?}", successful_parse_stmt4);
    println!("{:?}", successful_parse_stmt5);
    println!("{:?}", successful_parse_stmt6);

    println!("{:?}", successful_parse_stmt7);
    println!("{:?}", successful_parse_stmt8);
    println!("{:?}", successful_parse_stmt9);
    println!("{:?}", successful_parse_stmt10);
    println!("{:?}", successful_parse_stmt11);

    let inner_loop: types::WhileProgram = vec![
        types::WhilelangType {
            label: 4,
            statement: types::Statement {
                lValue: types::Stmt::Arithm(types::AExp::Var(types::Value::Var(String::from("z")))),
                rValue: Some(types::Stmt::Arithm(types::AExp::AritheticOp(
                    types::A1OpaA2 {
                        a1: types::Value::Var(String::from("z")),
                        a2: types::Value::Var(String::from("y")),
                        opa: types::Opa::Mul,
                    },
                ))),
            },
        },
        types::WhilelangType {
            label: 5,
            statement: types::Statement {
                lValue: types::Stmt::Arithm(types::AExp::Var(types::Value::Var(String::from("y")))),
                rValue: Some(types::Stmt::Arithm(types::AExp::AritheticOp(
                    types::A1OpaA2 {
                        a1: types::Value::Var(String::from("y")),
                        a2: types::Value::Num(1),
                        opa: types::Opa::Min,
                    },
                ))),
            },
        },
    ];

    let factorial: types::WhileProgram = vec![
        types::WhilelangType {
            label: 1,
            statement: types::Statement {
                lValue: types::Stmt::Arithm(types::AExp::Var(types::Value::Var(String::from("y")))),
                rValue: Some(types::Stmt::Arithm(types::AExp::Var(types::Value::Var(
                    String::from("x"),
                )))),
            },
        },
        types::WhilelangType {
            label: 2,
            statement: types::Statement {
                lValue: types::Stmt::Arithm(types::AExp::Var(types::Value::Var(String::from("z")))),
                rValue: Some(types::Stmt::Arithm(types::AExp::Var(types::Value::Num(1)))),
            },
        },
        types::WhilelangType {
            label: 3,
            statement: types::Statement {
                lValue: types::Stmt::While(types::While {
                    cond: types::BExp::RelationalOp(types::A1OprA2 {
                        a1: types::Value::Var(String::from("y")),
                        a2: types::Value::Num(1),
                        opr: types::Opr::Gt,
                    }),
                    statements: inner_loop,
                }),
                rValue: None,
            },
        },
        types::WhilelangType {
            label: 2,
            statement: types::Statement {
                lValue: types::Stmt::Arithm(types::AExp::Var(types::Value::Var(String::from("y")))),
                rValue: Some(types::Stmt::Arithm(types::AExp::Var(types::Value::Num(0)))),
            },
        },
    ];
}
