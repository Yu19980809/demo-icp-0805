use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub enum PostType {
  Free,
  Silver,
  Gold,
  Platinum,
  Paid
}

#[derive(CandidType, Debug, Deserialize, Serialize)]
pub struct Post {
  id: u64,
  content: String,
  image: Option<String>,
  video: Option<String>,
  post_type: PostType,
  creator_id: Principal,
  likes: Vec<Principal>,
  comments: Vec<String>,
  created_at: u64,
}
