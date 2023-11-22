use miden::{Assembler, DefaultHost, ProvingOptions, StackInputs, ExecutionProof, ProgramInfo, StackOutputs};

pub fn fibonacci_assembler(n: u32) -> String{
    format!(
        "
        begin
            repeat.{}
                swap dup.1 add
            end
        end",
        n - 1
    )
}

struct Fibonacci{
    source: String
}
impl Default for Fibonacci{
    fn default() -> Self {
        Fibonacci { source: fibonacci_assembler(50) }
    }
}
impl Fibonacci{
    fn prove(&self){
        let program: miden::Program = Assembler::default().compile(&self.source).unwrap();
        let host: DefaultHost<miden::MemAdviceProvider> = DefaultHost::default();
        let stack_inputs: StackInputs = StackInputs::try_from_values([0,1]).unwrap();
        let (outputs, proof) = miden::prove(
            &program,
            stack_inputs,
            host,
            ProvingOptions::default(),
        )
        .unwrap();
        let stack = outputs.stack_truncated(1);
    }
    fn verify(&self, proof: ExecutionProof, program: miden::Program){
        
    }
}

#[test]
fn test_fibonacci(){
    // set the number of terms to compute
    let n = 50;

    // instantiate the default assembler and compile the program
    let source = fibonacci_assembler(50);
    let program = Assembler::default().compile(&source).unwrap();

    // initialize a default host (with an empty advice provider)
    let host = DefaultHost::default();

    // initialize the stack with values 0 and 1
    let stack_inputs = StackInputs::try_from_values([0, 1]).unwrap();

    // execute the program
    let (outputs, proof) = miden::prove(
        &program,
        stack_inputs.clone(),
        host,
        ProvingOptions::default(), // use default proving options
    )
    .unwrap();

    // fetch the stack outputs, truncating to the first element
    let stack = outputs.stack_truncated(1);

    // the output should be the 50th Fibonacci number
    assert_eq!(&[12586269025], stack);

    let is_valid = miden::verify(ProgramInfo::from(program), StackInputs::try_from_values([0, 1]).unwrap(),outputs,  proof);
    match is_valid{
        Ok(_) => println!("[OK] Execution was verified successfully!"),
        Err(_) => eprintln!("[Err] Failed to verify execution!")
    };
}