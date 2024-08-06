use crate::model::posts::Posts;
use candid::Principal;
use canister_state_macros::canister_state;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use utils::env::Environment;
use types::{
  BuildVersion, Cycles, Post, PostId, Timestamped, TimestampMillis
};

mod lifecycle;
mod memory;
mod model;

thread_local! {
  static WASM_VERSION: RefCell<Timestamped<BuildVersion>> = RefCell::default();
}

canister_state!(RuntimeState);

struct RuntimeState {
  env: Box<dyn Environment>,
  data: Data,
}

impl RuntimeState {
  pub fn new(env: Box<dyn Environment>, data: Data) -> Self {
    RuntimeState { env, data }
  }

  pub fn metrics(&self) -> Metrics {
    let now = self.env.now();
    Metrics {
      heap_memory_used: utils::memory::heap(),
      stable_memory_used: utils::memory::stable(),
      now,
      cycles_balance: self.env.cycles_balance(),
      wasm_version: WASM_VERSION.with_borrow(|v| **v),
      created: self.data.user_created,
    }
  }
}

#[derive(Deserialize, Serialize)]
struct Data {
  pub owner: Principal,
  pub posts: Posts,
  pub storage_limit: u64,
  pub user_created: TimestampMillis,
  pub rng_seed: [u8; 32],
}

impl Data {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    owner: Principal,
    now: TimestampMillis
  ) -> Self {
    Data {
      owner,
      posts: Posts::default(),
      storage_limit: 0,
      user_created: now,
      rng_seed: [0; 32],
    }
  }

  // pub fn remove_post(&mut self, post_id: PostId, now: TimestampMillis) -> Option<Post> {
  //   let post = self.posts.remove(post_id, now)?;
  //   Some(post)
  // }
}

#[derive(Debug, Serialize)]
pub struct Metrics {
  pub now: TimestampMillis,
  pub heap_memory_used: u64,
  pub stable_memory_used: u64,
  pub cycles_balance: Cycles,
  pub wasm_version: BuildVersion,
  pub created: TimestampMillis,
}
