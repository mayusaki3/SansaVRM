// crates/sansavrm-core/src/lib.rs

//! SansaVRM Core.
//!
//! 役割:
//! - SansaVRM の内部メタモデルを提供する。
//! - Model / Module / Slot / State / Property / Diagnostics の基本構造を定義する。
//! - JSON Schema が定義する型・制約・参照構造の前提となる公開データ構造を提供する。
//! - conversion report や generated artifact metadata 等の共通変換情報を提供する。
//! - custom parameter schema や io_scope 等の変換拡張情報を提供する。
//!
//! 注意:
//! - Core crate は SansaVRM メタモデルと Core API の公開境界を提供する。
//! - JSON Schema 実体は schemas 配下に配置されるが、trace_check の実装参照対象外であるため、
//!   Core crate の公開境界に代表参照を置く。
//!
//! @hldocs.ref doc-20260504-000201Z-SV0A#sec_a8k3m2q1
//! @hldocs.ref doc-20260504-000204Z-SV0E#sec_b7n4p9r2
//! @hldocs.ref doc-20260504-000204Z-SV0E#sec_d0c8f3u8
//! @hldocs.ref doc-20260504-000204Z-SV0E#sec_d5w6x7u4
//! @hldocs.ref doc-20260504-000204Z-SV0E#sec_f3a8b5w6
//! @hldocs.ref doc-20260504-000206Z-SV0G#sec_k8j2m1a0
//! @hldocs.ref doc-20260504-000405Z-SV0S#sec_y5x3a8p4
//! @hldocs.ref doc-20260504-000209Z-SV0J#sec_w7v5y0m2

pub mod conversion_report;
pub mod custom_parameter;
pub mod custom_parameter_registry;
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

pub use conversion_report::*;
pub use custom_parameter::*;
pub use custom_parameter_registry::*;
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
