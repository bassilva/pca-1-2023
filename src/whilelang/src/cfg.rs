use super::types;

pub struct Cfg {
    graph: Vec<(i32, i32)>,
    program: types::WhileProgram,
}

impl Cfg {
    pub fn new(program: types::WhileProgram) -> Self {
        Cfg {
            graph: Vec::new(),
            program: program,
        }
    }

    pub fn build(&mut self) {
        for i in 0..self.program.len() {
            let label_1: i32;
            let label_2: i32;
            if i < self.program.len() - 1 {
                label_1 = self.program[i].label;
                label_2 = self.program[i + 1].label;
            } else {
                label_1 = self.program[i - 1].label;
                label_2 = self.program[i].label;
            }
            let statement = &&self.program[i].statement;
            match statement {
                types::Statement::Assignment(_) => self.graph.push((label_1, label_2)),
                types::Statement::Boolean(_) => self.graph.push((label_1, label_2)),
                types::Statement::Skip => self.graph.push((label_1, label_2)),
                types::Statement::While(w) => {
                    let initial_label = w.statements[0].label;
                    if i < self.program.len() - 1 {
                        self.graph.push((label_1, initial_label));
                    } else {
                        self.graph.push((label_2, initial_label));
                    }
                    for i in 0..w.statements.len() - 1 {
                        let new_label_1 = w.statements[i].label;
                        let new_label_2 = w.statements[i + 1].label;
                        self.graph.push((new_label_1, new_label_2));
                    }
                    let final_label = w.statements[w.statements.len() - 1].label;
                    if i < self.program.len() - 1 {
                        self.graph.push((final_label, label_1));
                    } else {
                        self.graph.push((final_label, label_2));
                    }
                }
            }
        }
    }

    pub fn get_graph(&self) -> &Vec<(i32, i32)> {
        &self.graph
    }
}

#[cfg(test)]
mod test_cfg {

    #[test]
    fn test_cfg_build_factorial() {
        let inner_loop: super::types::WhileProgram = vec![
            super::types::WhilelangType {
                label: 4,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("z"),
                    rValue: super::types::AExp::ArithmeticOp(super::types::A1OpaA2 {
                        a1: super::types::Value::Var(String::from("z")),
                        a2: super::types::Value::Var(String::from("y")),
                        opa: super::types::Opa::Mul,
                    }),
                }),
            },
            super::types::WhilelangType {
                label: 5,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("y"),
                    rValue: super::types::AExp::ArithmeticOp(super::types::A1OpaA2 {
                        a1: super::types::Value::Var(String::from("y")),
                        a2: super::types::Value::Num(1),
                        opa: super::types::Opa::Min,
                    }),
                }),
            },
        ];

        let factorial: super::types::WhileProgram = vec![
            super::types::WhilelangType {
                label: 1,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("y"),
                    rValue: super::types::AExp::Var(String::from("x")),
                }),
            },
            super::types::WhilelangType {
                label: 2,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("z"),
                    rValue: super::types::AExp::Num(1),
                }),
            },
            super::types::WhilelangType {
                label: 3,
                statement: super::types::Statement::While(super::types::While {
                    cond: super::types::BExp::RelationalOp(super::types::A1OprA2 {
                        a1: super::types::Value::Var(String::from("y")),
                        a2: super::types::Value::Num(1),
                        opr: super::types::Opr::Gt,
                    }),
                    statements: inner_loop,
                }),
            },
            super::types::WhilelangType {
                label: 6,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("y"),
                    rValue: super::types::AExp::Num(0),
                }),
            },
        ];
        let expected: Vec<(i32, i32)> = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 3), (3, 6)];

        let mut cfg = super::Cfg::new(factorial);
        cfg.build();
        let obtained = cfg.get_graph();

        assert_eq!(obtained, &expected);
    }

    #[test]
    fn test_cfg_build_power() {
        let inner_loop: super::types::WhileProgram = vec![
            super::types::WhilelangType {
                label: 3,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("z"),
                    rValue: super::types::AExp::ArithmeticOp(super::types::A1OpaA2 {
                        a1: super::types::Value::Var(String::from("z")),
                        a2: super::types::Value::Var(String::from("y")),
                        opa: super::types::Opa::Mul,
                    }),
                }),
            },
            super::types::WhilelangType {
                label: 4,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("x"),
                    rValue: super::types::AExp::ArithmeticOp(super::types::A1OpaA2 {
                        a1: super::types::Value::Var(String::from("x")),
                        a2: super::types::Value::Num(1),
                        opa: super::types::Opa::Min,
                    }),
                }),
            },
        ];

        let power: super::types::WhileProgram = vec![
            super::types::WhilelangType {
                label: 1,
                statement: super::types::Statement::Assignment(super::types::Assignment {
                    lValue: String::from("z"),
                    rValue: super::types::AExp::Num(1),
                }),
            },
            super::types::WhilelangType {
                label: 2,
                statement: super::types::Statement::While(super::types::While {
                    cond: super::types::BExp::RelationalOp(super::types::A1OprA2 {
                        a1: super::types::Value::Var(String::from("x")),
                        a2: super::types::Value::Num(0),
                        opr: super::types::Opr::Gt,
                    }),
                    statements: inner_loop,
                }),
            },
        ];
        let expected: Vec<(i32, i32)> = vec![(1, 2), (2, 3), (3, 4), (4, 2)];

        let mut cfg = super::Cfg::new(power);
        cfg.build();
        let obtained = cfg.get_graph();

        assert_eq!(obtained, &expected);
    }
}
