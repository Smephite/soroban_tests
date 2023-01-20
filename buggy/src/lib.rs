#![no_std]

mod generated {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/soroban_error_types.wasm"
    );
}
