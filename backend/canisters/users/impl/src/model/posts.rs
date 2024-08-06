use crate::model::post::Post;
use candid::CandidType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use types::{PostId, TimestampMillis};
use utils::env::Environment;

#[derive(Default, Deserialize, Serialize)]
pub struct Posts {
  posts: HashMap<PostId, Post>,
  removed: Vec<RemovedPost>,
  latest_post_index: u64,
}

#[derive(Deserialize, Serialize)]
struct RemovedPost {
  post_id: PostId,
  timestamp: TimestampMillis,
}

impl Posts {
  // pub fn create(
  //   &mut self,
  //   content: String,
  //   image: Option<String>,
  //   video: Option<String>,
  //   post_type: PostType,
  //   env: Box<dyn Environment>
  // ) -> bool {
  //   let index = self.latest_post_index + 1;
  //   self.posts.push(Post {
  //     id: index,
  //     content,
  //     image,
  //     video,
  //     post_type,
  //     creator_id: env.caller(),
  //     likes: Vec::new(),
  //     comments: Vec::new(),
  //     created_at: env.now(),
  //   });

  //   self.latest_post_index = index;
  //   true
  // }

  // pub fn remove(&mut self, post_id: PostId, now: TimestampMillis) -> Option<Post> {
  //   let post = self.posts.remove(&post_id);
  //   self.removed.push(RemovedPost { post_id, timestamp: now });
  //   Some(post)
  // }
}
