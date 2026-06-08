use crate::models::claims::Claims;
use crate::utils::jwt::verify_token;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};

/// 元组结构体
pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection>
    {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "缺少 Authorization 请求头"))?;
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or((StatusCode::UNAUTHORIZED, "Authorization 格式应为 Bearer <token>"))?;
        let claims = verify_token(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "token 无效或已过期"))?;
        Ok(AuthUser(claims))
    }
}