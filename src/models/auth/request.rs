use serde::Deserialize;


// 注册的请求
// f -> b 反序列化
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

// 登录的请求
#[derive(Deserialize)]
pub struct  LoginRequest{
    pub username:String,
    pub password:String
}

