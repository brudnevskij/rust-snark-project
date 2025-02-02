use crate::circuit::{Circuit, Gate};
use num_bigint::ToBigInt;

mod circuit;
mod hash_function;
mod merkle;
mod r1cs;

fn addition_proof() {
    let mut circuit = Circuit::new(None);

    let input1 = circuit.add_input(10.to_bigint().unwrap());
    let input2 = circuit.add_input(20.to_bigint().unwrap());

    let output_index = circuit.add_input(30.to_bigint().unwrap());
    circuit.add_gate(Gate::Add(input1, input2, output_index));
    circuit.set_output(30.to_bigint().unwrap());

    println!("Generate Addition Proof...");
    circuit.generate_proof("addition_proof.bin");
    let is_valid = circuit.verify_proof("addition_proof.bin");
    println!("Addition Proof is valid: {}", is_valid);
}

fn addition_proof_fail() {
    let mut circuit = Circuit::new(None);

    let input1 = circuit.add_input(10.to_bigint().unwrap());
    let input2 = circuit.add_input(20.to_bigint().unwrap());

    let output_index = circuit.add_input(31.to_bigint().unwrap());
    circuit.add_gate(Gate::Add(input1, input2, output_index));
    circuit.set_output(31.to_bigint().unwrap());

    println!("Generate Addition Proof...");
    circuit.generate_proof("addition_proof_fail.bin");
    let is_valid = circuit.verify_proof("addition_proof_fail.bin");
    println!("Addition Proof is valid: {}", is_valid);
}
fn main() {
    addition_proof();
    addition_proof_fail();
}
