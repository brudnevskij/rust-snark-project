use num_bigint::ToBigInt;

pub mod circuit;
pub mod hash_function;
pub mod merkle;
pub mod r1cs;

fn main() {}

#[cfg(test)]
mod test {
    use crate::circuit::{Circuit, Gate};
    use crate::hash_function::CustomHash;
    use crate::merkle::MerkleTree;
    use num_bigint::ToBigInt;

    fn clean_up_proof_file(path: &str) {
        std::fs::remove_file(path).expect(&format!("Failed removing file: {}", path));
    }
    #[test]
    fn addition_proof() {
        let proof_file_name = "addition_proof.bin";
        let mut circuit = Circuit::new(None);

        let input1 = circuit.add_input(10.to_bigint().unwrap());
        let input2 = circuit.add_input(20.to_bigint().unwrap());

        let output_index = circuit.add_input(30.to_bigint().unwrap());
        circuit.add_gate(Gate::Add(input1, input2, output_index));
        circuit.set_output(30.to_bigint().unwrap());

        println!("Generate Addition Proof...");
        circuit.generate_proof(proof_file_name);
        let is_valid = circuit.verify_proof(proof_file_name);
        clean_up_proof_file(proof_file_name);

        assert!(is_valid, "Addition Proof is not valid");
    }

    #[test]
    fn addition_proof_fail() {
        let proof_file_name = "addition_proof_fail.bin";
        let mut circuit = Circuit::new(None);

        let input1 = circuit.add_input(10.to_bigint().unwrap());
        let input2 = circuit.add_input(20.to_bigint().unwrap());

        let output_index = circuit.add_input(31.to_bigint().unwrap());
        circuit.add_gate(Gate::Add(input1, input2, output_index));
        circuit.set_output(31.to_bigint().unwrap());

        println!("Generate Addition Proof...");
        circuit.generate_proof(proof_file_name);
        let is_valid = circuit.verify_proof(proof_file_name);
        clean_up_proof_file(proof_file_name);

        assert!(!is_valid, "Addition Proof is valid");
    }

    #[test]
    fn multiplication_proof() {
        let proof_file_name = "multiplication_proof.bin";
        let mut circuit = Circuit::new(None);

        let input1 = circuit.add_input(10.to_bigint().unwrap());
        let input2 = circuit.add_input(20.to_bigint().unwrap());

        let output_index = circuit.add_input(200.to_bigint().unwrap());
        circuit.add_gate(Gate::Mul(input1, input2, output_index));
        circuit.set_output(200.to_bigint().unwrap());

        println!("Generate Addition Proof...");
        circuit.generate_proof(proof_file_name);
        let is_valid = circuit.verify_proof(proof_file_name);
        clean_up_proof_file(proof_file_name);

        assert!(is_valid, "Multiplication Proof is not valid");
    }

    #[test]
    fn multiplication_proof_fail() {
        let proof_file_name = "multiplication_proof_fail.bin";
        let mut circuit = Circuit::new(None);

        let input1 = circuit.add_input(10.to_bigint().unwrap());
        let input2 = circuit.add_input(20.to_bigint().unwrap());

        let output_index = circuit.add_input(2000.to_bigint().unwrap());
        circuit.add_gate(Gate::Mul(input1, input2, output_index));
        circuit.set_output(2000.to_bigint().unwrap());

        println!("Generate Addition Proof...");
        circuit.generate_proof(proof_file_name);
        let is_valid = circuit.verify_proof(proof_file_name);
        clean_up_proof_file(proof_file_name);

        assert!(!is_valid, "Multiplication Proof is valid");
    }

    #[test]
    fn merkle_tree_proof() {
        let proof_file_name = "merkle_tree_proof.bin";
        let transactions = vec![
            10.to_bigint().unwrap(),
            20.to_bigint().unwrap(),
            50.to_bigint().unwrap(),
            80.to_bigint().unwrap(),
        ];

        let merkle_tree = MerkleTree::new(transactions.clone(), CustomHash);
        let leaf_index = 2;
        let leaf_value = transactions[leaf_index].clone();
        let merkle_path = merkle_tree.merkle_path(leaf_index);

        let mut circuit = Circuit::new(Some(Box::new(CustomHash)));
        let leaf_index_var = circuit.add_input(leaf_value);
        let mut current_hash_index = leaf_index_var;

        for (sibling_hash, is_left) in merkle_path {
            let sibling_index_var = circuit.add_input(sibling_hash.clone());

            let new_hash_value = if is_left {
                circuit.apply_hash(
                    circuit
                        .get_input(sibling_index_var)
                        .expect("Invalid input index"),
                    circuit
                        .get_input(current_hash_index)
                        .expect("Invalid input index"),
                )
            } else {
                circuit.apply_hash(
                    circuit
                        .get_input(current_hash_index)
                        .expect("Invalid input index"),
                    circuit
                        .get_input(sibling_index_var)
                        .expect("Invalid input index"),
                )
            };

            println!(
                "Merkle proof step: current_hash = {}, sibling_hash = {}, new_hash = {}",
                circuit.get_input(current_hash_index).unwrap(),
                sibling_hash,
                new_hash_value
            );

            let new_hash_index = circuit.add_input(new_hash_value.clone());
            circuit.set_output(new_hash_value.clone());

            circuit.add_gate(if is_left {
                Gate::Hash(sibling_index_var, current_hash_index, new_hash_index)
            } else {
                Gate::Hash(current_hash_index, sibling_index_var, new_hash_index)
            });

            current_hash_index = new_hash_index;
        }

        circuit.set_output(merkle_tree.root.clone());

        println!("Expected Merkle root: {}", merkle_tree.root);
        circuit.generate_proof(proof_file_name);
        let is_valid = circuit.verify_proof(proof_file_name);
        clean_up_proof_file(proof_file_name);
        assert!(is_valid, "Merkle Tree proof is not valid")
    }
}
