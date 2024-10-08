use crate::env::Environment;
use crate::time;
use candid::Principal;
use rand::rngs::StdRng;
use rand::SeedableRng;
use types::{CanisterId, Cycles, TimestampNanos};

pub struct CanisterEnv {
  rng: StdRng
}

impl CanisterEnv {
  pub fn new(seed: [u8; 32]) -> Self {
    CanisterEnv {
      rng: StdRng::from_seed(seed)
    }
  }
}

impl Environment for CanisterEnv {
  fn now_nanos(&self) -> TimestampNanos {
    time::now_nanos()
  }

  fn caller(&self) -> Principal {
    ic_cdk::caller()
  }

  fn canister_id(&self) -> CanisterId {
    ic_cdk::id()
  }

  fn cycles_balance(&self) -> Cycles {
    ic_cdk::api::canister_balance().into()
  }

  fn rng(&mut self) -> &mut StdRng {
    &mut self.rng
  }
}

impl Default for CanisterEnv {
  fn default() -> Self {
    // let seed = CanisterEnv::new([0; 32]).entropy();
    let seed = [0; 32];
    CanisterEnv::new(seed)
  }
}
