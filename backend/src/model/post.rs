use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// 用户与非遗项目关联的文章
#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct Post {
    pub post_id: i64,                         // 文章ID（主键）
    pub user_id: i64,                         // 关联的用户ID（外键）
    pub ich_id: i64,                          // 关联的非遗项目ID（外键）
    pub post_title: String,                   // 文章标题
    pub post_content: String,                 // 文章内容（富文本或Markdown格式）
    pub post_media_urls: Option<Vec<String>>, // 文章图片/视频URL
    pub post_type: i8,                        // 文章类型（1=传承文章 2=研究记录 3=体验分享 4=其他）
    pub post_status: i8,                      // 文章状态（0=待审核 1=已发布 2=已下架 3=违规删除）
    pub post_likes: i32,                      // 点赞数
    pub post_favorites: i32,                  // 收藏数
    pub post_create_time: DateTime<Utc>,      // 创建时间
    pub post_update_time: DateTime<Utc>,      // 更新时间
}

#[derive(Debug, Deserialize)]
pub struct PostCreatePayload {
    pub ich_id: i64,
    pub post_type: i8,
    pub post_title: String,
    pub post_content: String,
    pub post_media_urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct PostSearchParam {
    pub keyword: String,
    pub page: u32,
    pub size: u32,
}

#[derive(Debug, Deserialize)]
pub struct PostUpdatePayload {
    pub post_title: String,
    pub post_content: String,
    pub post_media_urls: Option<Vec<String>>,
    pub post_type: i8,
}
