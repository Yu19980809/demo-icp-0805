use candid::CandidType;
use serde::Deserialize;
use types::PostType;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
  pub content: String,
  pub image: Option<String>,
  pub video: Option<String>,
  pub post_type: PostType,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
  Success
}
