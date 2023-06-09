use super::types;
use std::collections::HashMap;

pub struct WhileProgram {
    current_label: i32,
    temp_label: i32,
    storage: HashMap<i32, Vec<types::WhilelangType>>,
}

impl WhileProgram {
    pub fn new() -> Self {
        WhileProgram {
            current_label: 0,
            temp_label: 0,
            storage: HashMap::new(),
        }
    }

    pub fn add_assignment(&mut self, l_value: String, r_value: types::AExp) {
        self.current_label += 1;
        self.storage.entry(self.current_label).or_insert(Vec::new());
        if let Some(assignment) = self.storage.get_mut(&self.current_label) {
            assignment.push(types::WhilelangType {
                label: self.current_label,
                statement: types::Statement::Assignment(types::Assignment {
                    lValue: l_value,
                    rValue: r_value,
                }),
            });
        }
    }

    pub fn add_assignment_from_label(
        &mut self,
        l_value: String,
        r_value: types::AExp,
        from_label: i32,
    ) {
        if let Some(assignment) = self.storage.get_mut(&from_label) {
            self.current_label += 1;
            assignment.push(types::WhilelangType {
                label: self.current_label,
                statement: types::Statement::Assignment(types::Assignment {
                    lValue: l_value,
                    rValue: r_value,
                }),
            });
        }
    }

    pub fn add_while(&mut self, cond: types::BExp) -> i32 {
        self.current_label += 1;
        self.storage.entry(self.current_label).or_insert(Vec::new());
        if let Some(assignment) = self.storage.get_mut(&self.current_label) {
            assignment.push(types::WhilelangType {
                label: self.current_label,
                statement: types::Statement::While(types::While {
                    cond: cond,
                    statements: Vec::new(), // TODO change it to an integer
                }),
            });
        }

        self.current_label
    }
}
