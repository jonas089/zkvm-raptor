#![no_main]
#![no_std]

extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec
};
use casper_contract::{
    contract_api::{
        runtime::{self, put_key},
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, contracts::NamedKeys, runtime_args, ApiError, CLType,
    CLTyped, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    Parameter, RuntimeArgs, URef, U256, ContractPackage, ContractPackageHash
};
use miden::{Assembler, DefaultHost, ProvingOptions, StackInputs, ExecutionProof, ProgramInfo, StackOutputs};

#[no_mangle]
pub extern "C" fn verify(){
    // parse serialized inputs
    let proof_serialized: Vec<u8> = runtime::get_named_arg("proof");
    let outputs_stack: Vec<u64> = runtime::get_named_arg("outputs");
    let program_string: String = runtime::get_named_arg("program");
    // deserialize verifier inputs
    let program: miden::Program = Assembler::default().compile(program_string).unwrap();
    let outputs: StackOutputs = StackOutputs::new(outputs_stack, Vec::new()).unwrap();
    let proof: ExecutionProof = ExecutionProof::from_bytes(&proof_serialized).unwrap();
    // run verifier
    let is_valid: Result<u32, miden::VerificationError> = miden::verify(ProgramInfo::from(program), StackInputs::try_from_values([0, 1]).unwrap(),outputs,  proof);
    match is_valid{
        Ok(_) => {
            
            //runtime::print("[Ok] Execution proof was verified successfully!");
        },
        // replace with custom error
        Err(_) => {
            runtime::revert(ApiError::InvalidArgument);
            //runtime::print("[Err] Execution proof was NOT verified!");
        }
    };
}

#[no_mangle]
pub extern "C" fn call(){
    // entry point definitions
    let mut entry_points: EntryPoints = EntryPoints::new();
    let verify_proof: EntryPoint = EntryPoint::new(
        "verify",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );
    entry_points.add_entry_point(verify_proof);
    // named keys definitions
    let mut named_keys = NamedKeys::new();
    // contract package
    let package_key_name = "maiden_fibonacci_contract".to_string();
    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_key_name),
        Some("miden_fibonacci_access_key".to_string()),
    );
    let contract_hash_key = Key::from(contract_hash);
    // store contract package key
    runtime::put_key("maiden_fibonacci_contract", contract_hash_key);
}