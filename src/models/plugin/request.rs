
use serde::{Deserialize, Serialize};

// 序列化 就是 结构体 到二进制

/**
 * plugin 插件组件信息
 * Deserialize 是为了 从 json 里面读取的时候 能够 正确解析
 */
#[derive(Serialize,Deserialize)]
pub struct Plugin{
    pub id:String,
    pub name:String,
    pub version:String,
    pub download_url: String,
    pub readme_url: String,
}