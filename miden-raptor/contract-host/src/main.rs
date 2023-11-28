#![no_main]
#![no_std]

extern crate alloc;
use alloc::{
    string::{ToString},
    vec
};
use casper_contract::{
    contract_api::{
        runtime::{self},
        storage::{self},
    },
};
use casper_types::{
    contracts::NamedKeys, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
};
use casper_contract::contract_api::miden;
mod error;
use error::MidenError;

#[no_mangle]
pub extern "C" fn call_verifier(){
    let response: [u8; 1] = miden::miden_verifier();
    if response == [1u8]{
        Ok(())
    }
    else{
        runtime::revert(MidenError::InvalidProof);
    }
}

#[no_mangle]
pub extern "C" fn call(){
    // entry point definitions
    let mut entry_points: EntryPoints = EntryPoints::new();
    let call_verifier: EntryPoint = EntryPoint::new(
        "call_verifier",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract
    );
    entry_points.add_entry_point(call_verifier);
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