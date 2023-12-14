mod types;
use std::io::{BufReader, Cursor};
use std::path::PathBuf;
use ark_ec::bls12::{Bls12, Bls12Config};
use types::{CircomProof, Groth16Proof, Groth16VerifyingKey};
use std::collections::HashMap;

use ark_groth16::{Groth16, ProvingKey, PreparedVerifyingKey, Proof};
use ark_crypto_primitives::snark::SNARK;
use num_bigint::BigInt;
use ark_ec::{
    bn::Bn
};
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize, Write};
use ark_circom::{CircomConfig, CircomBuilder, CircomCircuit};
use ark_bls12_377::{Bls12_377, Config, G1Affine, G2Affine, FrConfig};
use std::io::{self, Read};
use std::fs::File;
use serde_json;
use serde::{Serialize, Deserialize};
use std::fs;
extern crate tempfile;
use tempfile::NamedTempFile;

type GrothBls = Groth16<Bls12_377>;

pub fn main(){

}

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

pub fn verify(
    circom_input: Vec<u8> 
) -> [u8;1]{
    let input: CircomInput = serde_json::from_slice(&circom_input.as_ref()).unwrap();
    let vk: ark_groth16::VerifyingKey<Bls12<Config>> = Groth16VerifyingKey { 
        alpha_g1: input.alpha_g1,
        beta_g2: input.beta_g2, 
        delta_g2: input.delta_g2,
        gamma_g2: input.gamma_g2, 
        gamma_abc_g1: input.gamma_abc_g1
    }.build();
    let proof: Groth16Proof = Groth16Proof{
        a: input.a,
        b: input.b,
        c: input.c
    };
    let pvk: ark_groth16::PreparedVerifyingKey<Bls12<Config>> = GrothBls::process_vk(&vk).unwrap();
    let mut wasm_file = NamedTempFile::new().unwrap();
    let mut r1cs_file = NamedTempFile::new().unwrap();
    let _ = wasm_file.write_all(&input.circuit_wasm);
    let _ = r1cs_file.write_all(&input.circuit_r1cs);
    wasm_file.flush().unwrap();
    r1cs_file.flush().unwrap();
    let wasm_path: tempfile::TempPath = wasm_file.into_temp_path();
    let r1cs_path: tempfile::TempPath = r1cs_file.into_temp_path();
    let cfg = CircomConfig::<Bls12<Config>>::new(
        wasm_path,
        r1cs_path
    ).unwrap();
    // Insert our public inputs as key value pairs
    let mut builder: CircomBuilder<Bls12<Config>> = CircomBuilder::new(cfg);
    if input.inputs.len() > 0{
        for (key, value) in input.inputs{
            builder.push_input(key, value);
        };
    }
    
    let circom: CircomCircuit<Bls12<Config>> = builder.build().unwrap();
    let inputs = circom.get_public_inputs().unwrap();
    // verify groth16 proof
    if GrothBls::verify_with_processed_vk(&pvk, &inputs, &proof.build()).unwrap() == true{
        [1]
    }
    else{
        [0]
    }
}


#[test]
fn test_verifier(){        
    // Load the WASM and R1CS for witness and proof generation
    let cfg: CircomConfig<Bls12<Config>> = CircomConfig::<Bls12_377>::new(
        "/users/chef/Desktop/zkvm-raptor/test-circom-host/circom/multiplier/multiplier.wasm",
        "/users/chef/Desktop/zkvm-raptor/test-circom-host/circom/multiplier/multiplier.r1cs",
    ).expect("Missing Circuit file(s)!");

    // Insert our public inputs as key value pairs
    let mut builder: CircomBuilder<Bls12<Config>> = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 11);
    builder.push_input("c", 33);

    // Create an empty instance for setting it up
    let circom: CircomCircuit<Bls12<Config>> = builder.setup();

    // Run a trusted setup
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let params: ProvingKey<Bls12<Config>> = GrothBls::generate_random_parameters_with_reduction(circom, &mut rng).unwrap();

    // Get the populated instance of the circuit with the witness
    let circom: CircomCircuit<Bls12<Config>> = builder.build().unwrap();

    let inputs = circom.get_public_inputs().unwrap();
    let mut buffer = Vec::new();
    let _ = inputs.iter().map(|input| input.0.serialize_uncompressed(&mut buffer));

    let deserialized_inputs = Vec::deserialize_uncompressed(&mut buffer.as_slice()).unwrap();

    // Generate the proof
    let proof: ark_groth16::Proof<Bls12<Config>> = GrothBls::prove(&params, circom, &mut rng).unwrap();

    // Check that the proof is valid
    let pvk: ark_groth16::PreparedVerifyingKey<Bls12<Config>> = GrothBls::process_vk(&params.vk).unwrap();

    /*
        Todo: serialize and deserialize the "pvk" -> easy
        Todo: serialize and deserialize the "inputs"
        Todo: serialize and deserialize the "proof" -> easy
    */
    /*
    let mut proof_serialized: Vec<u8> = Vec::new();
    let _ = &proof.serialize_compressed(proof_serialized).unwrap();
    let proof_deserialized: Proof<Bls12<Config>> = ?;
    */
    
    let mut serialized_pvk : Vec<u8> = Vec::new();
    let _ = &pvk.serialize_uncompressed(&mut serialized_pvk);
    let deserialized: PreparedVerifyingKey<Bls12<Config>> = PreparedVerifyingKey::deserialize_uncompressed(serialized_pvk.as_slice()).unwrap();

    //let serialized_proof: Vec<u8> = Vec::new();
    //let _ = &proof.serialize_uncompressed(&mut serialized_proof);
    //let deserialized_proof: Proof<Bls12<Config>> = Proof::deserialize_uncompressed(reader)


    let verified: bool = GrothBls::verify_with_processed_vk(&pvk, &deserialized_inputs, &proof).unwrap();
    assert!(verified);
}
