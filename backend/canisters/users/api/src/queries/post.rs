use candid::CandidType;
use serde::Deserialize;
use types::Post;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
  pub id: u64
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
  Success(SuccessResult),
  PostNotFound,
  NoAuthority,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
  pub data: Post
}
