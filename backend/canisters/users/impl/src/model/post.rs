use candid::Principal;
use serde::{Deserialize, Serialize};
use types::{CommentId, PostId, PostType, TimestampMillis, Timestamped};

#[derive(Deserialize, Serialize)]
pub struct Post {
  pub post_id: PostId,
  pub content: String,
  pub image: Option<String>,
  pub video: Option<String>,
  pub post_type: PostType,
  pub likes: Vec<Principal>,
  pub views: Vec<Principal>,
  pub comments: Vec<CommentId>,
  pub created_at: TimestampMillis,
}

impl Post {
  
}
