use crate::Data;
use crate::lifecycle::{init_env, init_state};
use ic_cdk::init;
use users_canister::init::Args;
use utils::env::Environment;

#[init]
fn init(args: Args) {
  let env = init_env([0; 32]);
  let now = env.now();
  let data = Data::new(args.owner, now);

  init_state(env, data, args.wasm_version);
}
