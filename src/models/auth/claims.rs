use serde::{Deserialize, Serialize};

/// JWT Payload 
/// - 注意: Claims 只做 base64 编码不加密, 任何人都能解开看, 故绝不放密码等敏感信息
/// - sub (subject): 主体, 这里放用户 id
/// - username: 用户名 (方便前端直接用, 非必须)
/// - exp (expiration): 过期时间 (Unix 秒级时间戳). jsonwebtoken 验证时会自动检查是否过期
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub exp: usize,
}
