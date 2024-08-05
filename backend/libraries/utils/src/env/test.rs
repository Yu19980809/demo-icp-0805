use crate::env::Environment;
use candid::Principal;
use types::{CanisterId, Cycles, TimestampMillis};

#[cfg(test)]
pub struct TestEnv {
  pub now: u64,
  pub caller: Principal,
  pub canister_id: Principal,
  pub cycles_balance: Cycles,
}

impl Environment for TestEnv {
  fn now(&self) -> TimestampMillis {
    self.now
  }

  fn caller(&self) -> Principal {
    self.caller
  }

  fn canister_id(&self) -> CanisterId {
    self.canister_id
  }

  fn cycles_balance(&self) -> Cycles {
    self.cycles_balance
  }
}

impl Default for TestEnv {
  fn default() -> Self {
    TestEnv {
      now: 1,
      caller: Principal::anonymous(),
      canister_id: Principal::anonymous(),
      cycles_balance: 1_000_000_000_000,
    }
  }
}
