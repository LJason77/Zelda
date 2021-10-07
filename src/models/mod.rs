use serde::{Deserialize, Serialize};

/// 对象结果
///
/// 返回给前端的对象结果
#[derive(Default, Serialize, Deserialize)]
pub struct Results<'a, T> {
    /// 状态码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u8>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// 信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<&'a str>,
}
