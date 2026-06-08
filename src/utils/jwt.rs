use crate::models::claims::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error as JwtError,
};

/// token 有效期 (小时)
const TOKEN_EXPIRE_HOURS: i64 = 24 * 7;

/// 从环境变量读取 JWT 密钥
fn get_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET 未在 .env 中配置")
}

/// 签发 token
pub fn create_token(user_id: i32, username: &str) -> Result<String, JwtError> {
    let exp = (Utc::now() + Duration::hours(TOKEN_EXPIRE_HOURS)).timestamp() as usize;
    let claims: Claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_secret().as_bytes()),
    )
}

/// 验证token
pub fn verify_token(token: &str) -> Result<Claims, JwtError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(get_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}
