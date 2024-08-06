use crate::lifecycle::{init_env, init_state};
use crate::memory::get_upgrades_memory;
use crate::Data;
use ic_cdk::post_upgrade;
use stable_memory::get_reader;
use types::Milliseconds;
use users_canister::post_upgrade::Args;
use utils::time::DAY_IN_MS;

const SIX_MONTHS: Milliseconds = 183 * DAY_IN_MS;

#[post_upgrade]
fn post_upgrade(args: Args) {
  let memory = get_upgrades_memory();
  let reader = get_reader(&memory);

  let (data,): (Data,) = serializer::deserialize(reader).unwrap();

  let env = init_env(data.rng_seed);
  init_state(env, data, args.wasm_version);
}
