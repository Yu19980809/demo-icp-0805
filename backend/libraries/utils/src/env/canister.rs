use candid::Principal;
use types::{CanisterId, Cycles, TimestampMillis};

use super::Environment;

pub struct CanisterEnv {}

impl CanisterEnv {
  pub fn new() -> Self {
    CanisterEnv {}
  }
}

impl Environment for CanisterEnv {
  fn now(&self) -> TimestampMillis {
    ic_cdk::api::time()
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
}
