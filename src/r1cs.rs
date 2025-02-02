use num_bigint::{BigInt, BigUint};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process::Output;

#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    pub index: usize,
    pub value: BigInt,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Add,
    Mul,
    Hash,
}

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    pub left: Vec<(Variable, BigInt)>,
    pub right: Vec<(Variable, BigInt)>,
    pub output: Vec<(Variable, BigInt)>,
    pub operation: Operation,
}

#[derive(Serialize, Deserialize)]
pub struct R1CS {
    pub variables: Vec<Variable>,
    pub constraints: Vec<Constraint>,
}

impl R1CS {
    pub fn new() -> Self {
        R1CS {
            variables: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(
        &mut self,
        left: Vec<(Variable, BigInt)>,
        right: Vec<(Variable, BigInt)>,
        output: Vec<(Variable, BigInt)>,
        operation: Operation,
    ) {
        self.constraints.push(Constraint {
            left,
            right,
            output,
            operation,
        })
    }

    pub fn is_satisfied<F>(&self, apply_hash: F) -> bool
    where
        F: Fn(&BigInt, &BigInt) -> BigInt,
    {
        for constraint in &self.constraints {
            let left_value: BigInt = constraint
                .left
                .iter()
                .map(|(variable, coefficient)| &variable.value * coefficient)
                .sum();
            let right_value: BigInt = constraint
                .right
                .iter()
                .map(|(variable, coefficient)| &variable.value * coefficient)
                .sum();
            let output_value: BigInt = constraint
                .output
                .iter()
                .map(|(variable, coefficient)| &variable.value * coefficient)
                .sum();

            match constraint.operation {
                Operation::Add => {
                    let sum = left_value + right_value;
                    if sum != output_value {
                        println!(
                            "Addition constraint not satisfied: left + right = {}, but output = {}",
                            sum, output_value
                        );
                        return false;
                    }
                }
                Operation::Mul => {
                    let product = left_value * right_value;
                    if product != output_value {
                        println!(
                            "Multiplication constraint not satisfied: left * right = {}, but output = {}",
                            product,
                            output_value
                        );
                        return false;
                    }
                }
                Operation::Hash => {
                    let computed_hash = apply_hash(&left_value, &right_value);
                    if computed_hash != output_value {
                        println!(
                            "Hash constraint not satisfied: hash_function(left, right) = {}, but output = {}",
                            computed_hash,
                            output_value
                        );
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn save_to_binary(&self, file_name: &str) {
        let mut file = File::create(file_name).expect("Could not create a file");
        let data = bincode::serialize(self).expect("Failed to serialize R1CS");
        file.write_all(&data).expect("Failed to write to a file");
    }
}
