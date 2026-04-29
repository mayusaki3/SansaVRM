// crates/sansavrm-core/src/lib.rs

//! SansaVRM Core.
//!
//! 役割:
//! - SansaVRM の内部メタモデルを提供する。
//! - Model / Module / Slot / State / Property / Diagnostics の基本構造を定義する。
//!
//! 注意:
//! - 正式な HLDocS `@hldocs.ref` は HLDocS 化後に付与する。
//! - 現段階では `TODO(trace)` による暫定トレースのみを使用する。

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
