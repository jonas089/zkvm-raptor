# Casper Zk research -> Ext_ffi & contracts
<img src="https://github.com/jonas089/polygon-raptor/blob/master/resources/logo.webp" alt="Alternative Text" width="500" height="500">


## casper-circom
Smart contract that calls the experimental host function "circom_verifier" -> contains a valid payload for the `multiplier2` circuit.

## miden-raptor/contract
Smart contract that calls the miden verifier for the `fibonacci` example program => execution cost ~18 CSPR

## miden-raptor/contract-host
Smart contract that calls the experimental host function "miden-verifier" => execution cost ~ 0.0000002 CSPR (hardcoded)

## proving-system and proving-system-wasm
Off-chain tests for miden contracts

## test-circom-host
Off-chain tests for circom verifier implementation