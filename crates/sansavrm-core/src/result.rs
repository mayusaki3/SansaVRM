// crates/sansavrm-core/src/result.rs

use crate::SansaVrmError;

/// Core API 共通 Result。
///
/// 役割:
/// - 成功値、エラー、警告、情報をまとめて返す。
///
/// 注意:
/// - CoreAPI仕様の Result<T> に対応する。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub errors: Vec<SansaVrmError>,
    pub warnings: Vec<SansaVrmError>,
    pub infos: Vec<SansaVrmError>,
}

impl<T> CoreResult<T> {
    /// 成功結果を生成する。
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            errors: Vec::new(),
            warnings: Vec::new(),
            infos: Vec::new(),
        }
    }

    /// 失敗結果を生成する。
    pub fn fail(error: SansaVrmError) -> Self {
        Self {
            success: false,
            data: None,
            errors: vec![error],
            warnings: Vec::new(),
            infos: Vec::new(),
        }
    }
}
