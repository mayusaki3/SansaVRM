// crates/sansavrm-core/src/lib.rs

//! SansaVRM Core.
//!
//! 役割:
//! - SansaVRM の内部メタモデルを提供する。
//! - Model / Module / Slot / State / Property / Diagnostics の基本構造を定義する。
//!
//! 注意:
//! - Core crate は SansaVRM メタモデルと Core API の公開境界を提供する。
//!
//! @hldocs.ref doc-20260504-000201Z-SV0A#sec_a8k3m2q1
//! @hldocs.ref doc-20260504-000206Z-SV0G#sec_k8j2m1a0

pub mod diagnostics;
pub mod error;
pub mod id;
pub mod model;
pub mod module;
pub mod property;
pub mod result;
pub mod slot;
pub mod state;
pub mod connection_api;
pub mod property_api;
pub mod io_api;
pub mod state_api;
pub mod model_api;
pub mod module_api;
pub mod slot_api;
pub mod transaction_api;

pub use diagnostics::*;
pub use error::*;
pub use id::*;
pub use model::*;
pub use module::*;
pub use property::*;
pub use result::*;
pub use slot::*;
pub use state::*;
pub use connection_api::*;
pub use property_api::*;
pub use io_api::*;
pub use state_api::*;
pub use model_api::*;
pub use module_api::*;
pub use slot_api::*;
pub use transaction_api::*;
