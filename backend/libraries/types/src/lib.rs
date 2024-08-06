use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

mod build_version;
mod cycles;
mod posts;
mod timestamped;

pub use build_version::*;
pub use cycles::*;
pub use posts::*;
pub use timestamped::*;

pub type CanisterId = Principal;
pub type PostId = u64;
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;

#[derive(CandidType, Clone, Debug, Default, Deserialize, Serialize)]
pub struct Empty {}
