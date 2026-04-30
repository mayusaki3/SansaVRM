// crates/sansavrm-core/src/error.rs

use thiserror::Error;

/// SansaVRM Core 共通エラー。
///
/// 役割:
/// - Core API / Validator / Adapter から共通利用できるエラー型を定義する。
///
/// 注意:
/// - 詳細なエラーコード体系は Validator 実装時に拡張する。
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum SansaVrmError {
    /// 指定された ID が見つからない。
    #[error("ID not found: {0}")]
    IdNotFound(String),

    /// ID が重複している。
    #[error("Duplicate ID: {0}")]
    DuplicateId(String),

    /// 入力値が不正。
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
