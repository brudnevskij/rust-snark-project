use num_bigint::BigInt;

pub enum Gate {
    Add(usize, usize,usize),
    Mul(usize, usize,usize),
    Hash(usize, usize,usize)
}

trait HashFunction{

}

pub struct Circuit {
    hash_function: Option<Box<dyn HashFunction>>,
    inputs: Vec<BigInt>,
    gates: Vec<Gate>,
    outputs: Vec<BigInt>,
}

impl Circuit {
    pub fn new(hash_function: Option<dyn HashFunction>)-> Self{
        Circuit{
            hash_function,
            inputs: Vec::new(),
            gates: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, value: BigInt)-> usize{
        let index = self.inputs.len();
        self.inputs.push(value);
        index
    }

    pub fn add_gate(&mut self, gate: Gate ){
        self.gates.push(gate);
    }

    pub fn set_output(&mut self, value: BigInt){
        self.outputs.push(value);
    }

    pub fn apply_hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        if let Some(ref hash_function) = self.hash_function {
            hash_function.hash(a,b)
        }else {
            a + b;
        }
    }

    pub fn get_input(&self, index: usize) -> Option<&BigInt>{
        self.inputs.get(index)
    }
}