use crate::{mutate_state, read_state};
use ic_cdk::update;
use users_canister::delete_post::{Response::*, *};

#[update]
async fn delete_post(args: Args) -> Response {
  
}
