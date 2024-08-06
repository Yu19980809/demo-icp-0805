use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use types::BuildVersion;

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub owner: Principal,
    pub wasm_version: BuildVersion,
}