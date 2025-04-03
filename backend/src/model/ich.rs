use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Ich {
    pub ich_id: i64,                // 项目序号
    pub ich_code: String,           // 项目编号
    pub ich_name: String,           // 项目编号
    pub ich_publish: String,        // 公布时间
    pub ich_category: String,       // 项目类别
    pub ich_location: String,       // 所属地区
    pub ich_apply_location: String, // 申请地点
    pub ich_protect_unit: String,   // 保护单位
    pub ich_content: String,        // 项目内容
    pub ich_create: DateTime<Utc>,  // 项目添加到系统的时间
    pub ich_uploader: i64,          // 添加项目的用户
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IchCreatePayload {
    pub ich_code: String,
    pub ich_name: String,
    pub ich_publish: String,
    pub ich_category: String,
    pub ich_location: String,
    pub ich_apply_location: String,
    pub ich_protect_unit: String,
    pub ich_content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IchQueryParam {
    pub keyword: String, // 模糊查询的名称
    pub page: u32,       // 分页页码
    pub size: u32,       // 每页大小
}

#[derive(Debug, Deserialize)]
pub struct IchUpdatePayload {
    pub ich_code: String,
    pub ich_name: String,
    pub ich_publish: String,
    pub ich_category: String,
    pub ich_location: String,
    pub ich_apply_location: String,
    pub ich_protect_unit: String,
    pub ich_content: String,
}
