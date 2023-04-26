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

    println!("{:?}", successful_parse_num);
    println!("{:?}", successful_parse_var);
    println!("{:?}", successful_parse_lab);

    println!("{:?}", successful_parse_aexp1);
    println!("{:?}", successful_parse_aexp2);
    println!("{:?}", successful_parse_aexp3);

    println!("{:?}", successful_parse_bexp1);
    println!("{:?}", successful_parse_bexp2);
    println!("{:?}", successful_parse_bexp3);
}
