use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
  pub id: u64
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
  Success,
  PostNotFound,
  NoAuthority,
}
