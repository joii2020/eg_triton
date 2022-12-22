use triton_vm::{
    proof::Claim,
    proof::Proof,
    stark::{Stark, StarkParameters},
    vm::Program,
};
use twenty_first::shared_math::b_field_element::BFieldElement;

pub const BASIC_RAM_READ_WRITE: &str = concat!("push  5", "halt ",);

fn main() {
    let program = Program::from_code("push 5 write_io halt").expect("form code failed");
    let claim = Claim {
        input: vec![],
        program: program.to_bwords(),
        output: vec![BFieldElement::from(5u64)],
        padded_height: 0,
    };

    let parameters = StarkParameters::default();
    let stark = Stark::new(claim.clone(), parameters);

    let filename = "test.tsp";
    let proof = if std::path::PathBuf::from(filename).is_file() {
        let bin = std::fs::read(filename).unwrap();
        let proof: Proof = bincode::deserialize(&bin).unwrap();
        proof
    } else {
        let (aet, _, err) = program.simulate_no_input();
        if err.is_some() {
            panic!("{:?}", err);
        }
        let proof = stark.prove(aet, &mut None);
        std::fs::write(filename, bincode::serialize(&proof).unwrap())
            .expect("write proof file failed");
        proof
    };

    // verify
    let program2 = Program::from_code("").expect("form code failed");
    let claim2 = Claim {
        input: vec![],
        program: program2.to_bwords(),
        output: vec![BFieldElement::from(5u64)],
        padded_height: 0,
    };
    let stark2 = Stark::new(claim2, StarkParameters::default());
    let result = stark2.verify(proof.clone(), &mut None);
    if let Err(e) = result {
        panic!("The Verifier is unhappy! {}", e);
    }
    println!("success");
}
