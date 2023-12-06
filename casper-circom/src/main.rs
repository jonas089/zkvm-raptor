#![no_main]

extern crate alloc;
use alloc::{
    string::{ToString},
    vec
};
use casper_contract::{
    contract_api::{
        runtime::{self},
        storage::{self},
        circom::{circom_verifier, self}
    },
};
use casper_types::{
    contracts::NamedKeys, CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, ApiError,
};

mod error;
use error::CircomError;
mod proof;
use proof::{alpha_g1, beta_g2, delta_g2, gamma_g2, gamma_g1_abc_serialized, a, b, c, wasm, r1cs, inputs};

use serde_json_wasm;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct CircomInput{
    alpha_g1: Vec<u8>,
    beta_g2: Vec<u8>,
    delta_g2: Vec<u8>,
    gamma_g2: Vec<u8>,
    gamma_abc_g1: Vec<Vec<u8>>,
    a: Vec<u8>,
    b: Vec<u8>,
    c: Vec<u8>,
    circuit_wasm: Vec<u8>,
    circuit_r1cs: Vec<u8>,
    inputs: Vec<(String, i32)>
}

#[no_mangle]
pub extern "C" fn call_verifier(){
    let circom_input = CircomInput{
        alpha_g1: alpha_g1,
        beta_g2: beta_g2,
        delta_g2: delta_g2,
        gamma_g2: gamma_g2,
        gamma_abc_g1: gamma_g1_abc_serialized,
        a: a,
        b: b,
        c: c,
        circuit_wasm: wasm,
        circuit_r1cs: r1cs,
        inputs: inputs
    };

    let _result: [u8; 1] = circom_verifier(&serde_json_wasm::to_vec(&circom_input).unwrap());
    if _result != [1]{
        runtime::revert(CircomError::InvalidProof);
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
    let package_key_name = "circom_multiplier_contract".to_string();
    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(package_key_name),
        Some("circom_multiplier_contract".to_string()),
    );
    let contract_hash_key = Key::from(contract_hash);
    // store contract package key
    runtime::put_key("circom_multiplier_contract", contract_hash_key);
}