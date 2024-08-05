use candid::CandidType;
use serde::Deserialize;
use types::Post;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
  pub limit: u64,
  pub current_page: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
  Success(SuccessResult)
}

#[derive(CandidType, Deserialize, Debug)]
pub struct SuccessResult {
  pub limit: u64,
  pub current_page: u64,
  pub total_pages: u64,
  pub data: Vec<Post>,
}
