fn main(){
    panic!("Not implemented!");
}

#[test]
fn hello_fibonacci(){
    use miden::{Assembler, ProgramInputs, ProofOptions};

    // set the number of terms to compute
    let n = 50;

    // instantiate the default assembler and compile the program
    let source = format!(
        "
        begin 
            repeat.{}
                swap dup.1 add
            end
        end",
        n - 1
    );
    let program = Assembler::default().compile(&source).unwrap();

    // initialize the stack with values 0 and 1
    let inputs = ProgramInputs::from_stack_inputs(&[0, 1]).unwrap();

    // execute the program
    let (outputs, proof) = miden::prove(
        &program,
        &inputs,
        &ProofOptions::default(), // use default proof options
    )
    .unwrap();

    // the output should be the 50th Fibonacci number
    // assert_eq!(vec![12586269025], outputs.stack().);
    assert_eq!(&12586269025u64, outputs.stack().get(0).expect("Failed to get from output stack!"));
}