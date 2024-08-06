use crate::memory::get_upgrades_memory;
use crate::take_state;
use ic_cdk::pre_upgrade;
use stable_memory::get_writer;

#[pre_upgrade]
fn pre_upgrade() {
  let mut state = take_state();
  let mut memory = get_upgrades_memory();
  let stable_state = (&state.data,);
  let writer = get_writer(&mut memory);

  serializer::serialize(stable_state, writer).unwrap();
}
