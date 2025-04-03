use super::SqlResult;
use crate::{model::post::Post, util::error::AppError};
use chrono::Utc;
use sqlx::postgres::PgPool;

/// 创建文章记录
pub async fn create(
    pool: &PgPool,
    user_id: i64,
    ich_id: i64,
    post_title: &str,
    post_content: &str,
    post_media_urls: Option<Vec<String>>,
    post_type: i8,
) -> SqlResult<i64> {
    let sql = "
    INSERT INTO post (
        user_id, ich_id, post_title, post_content,
        post_media_urls, post_type, post_status,
        post_create_time, post_update_time
    ) VALUES ($1, $2, $3, $4, $5, $6, 0, $7, $8)
    RETURNING post_id";

    let now = Utc::now();
    let post_id = sqlx::query_scalar(sql)
        .bind(user_id)
        .bind(ich_id)
        .bind(post_title)
        .bind(post_content)
        .bind(post_media_urls)
        .bind(post_type)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;

    Ok(post_id)
}

/// 根据ID查询文章
pub async fn get_by_id(pool: &PgPool, post_id: i64) -> SqlResult<Post> {
    let sql = "SELECT * FROM post WHERE post_id = $1";

    let post_opt = sqlx::query_as::<_, Post>(sql)
        .bind(post_id)
        .fetch_optional(pool)
        .await?;

    match post_opt {
        Some(post) => Ok(post),
        None => Err(AppError::PostNotFound),
    }
}

/// 更新文章信息
pub async fn update(
    pool: &PgPool,
    post_id: i64,
    post_title: &str,
    post_content: &str,
    post_media_urls: Option<Vec<String>>,
    post_type: i8,
) -> SqlResult<bool> {
    let sql = "
    UPDATE post SET
        post_title = $1,
        post_content = $2,
        post_media_urls = $3,
        post_type = $4,
        post_update_time = $5
    WHERE post_id = $6";

    let now = Utc::now();
    let rows_affected = sqlx::query(sql)
        .bind(post_title)
        .bind(post_content)
        .bind(post_media_urls)
        .bind(post_type)
        .bind(now)
        .bind(post_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

/// 更新文章状态
pub async fn update_status(pool: &PgPool, post_id: i64, post_status: i8) -> SqlResult<bool> {
    let sql = "
    UPDATE post SET
        post_status = $1,
        post_update_time = $2
    WHERE post_id = $3";

    let now = Utc::now();
    let rows_affected = sqlx::query(sql)
        .bind(post_status)
        .bind(now)
        .bind(post_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

/// 删除文章
pub async fn delete(pool: &PgPool, post_id: i64) -> SqlResult<bool> {
    let sql = "DELETE FROM post WHERE post_id = $1";

    let rows_affected = sqlx::query(sql)
        .bind(post_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

/// 检查文章是否存在
pub async fn exists(pool: &PgPool, post_id: i64) -> SqlResult<bool> {
    let sql = "SELECT EXISTS(SELECT 1 FROM post WHERE post_id = $1)";

    let exists = sqlx::query_scalar(sql)
        .bind(post_id)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}

/// 根据用户ID查询文章列表（带分页）
pub async fn list_by_user(
    pool: &PgPool,
    user_id: i64,
    page: u32,
    size: u32,
) -> SqlResult<Vec<Post>> {
    let offset = (page - 1) * size;

    let sql = "
    SELECT * FROM post
    WHERE user_id = $1
    ORDER BY post_create_time DESC
    LIMIT $2 OFFSET $3";

    let posts = sqlx::query_as::<_, Post>(sql)
        .bind(user_id)
        .bind(size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

    Ok(posts)
}

/// 根据非遗项目ID查询文章列表（带分页）
pub async fn list_by_ich(pool: &PgPool, ich_id: i64, page: u32, size: u32) -> SqlResult<Vec<Post>> {
    let offset = (page - 1) * size;

    let sql = "
    SELECT * FROM post
    WHERE ich_id = $1
    ORDER BY post_create_time DESC
    LIMIT $2 OFFSET $3";

    let posts = sqlx::query_as::<_, Post>(sql)
        .bind(ich_id)
        .bind(size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

    Ok(posts)
}

/// 根据文章类型查询文章列表（带分页）
pub async fn list_by_type(
    pool: &PgPool,
    post_type: i8,
    page: u32,
    size: u32,
) -> SqlResult<Vec<Post>> {
    let offset = (page - 1) * size;

    let sql = "
    SELECT * FROM post
    WHERE post_type = $1
    ORDER BY post_create_time DESC
    LIMIT $2 OFFSET $3";

    let posts = sqlx::query_as::<_, Post>(sql)
        .bind(post_type)
        .bind(size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

    Ok(posts)
}

/// 根据标题模糊查询文章（带分页）
pub async fn search_by_title(
    pool: &PgPool,
    title: &str,
    page: u32,
    size: u32,
) -> SqlResult<Vec<Post>> {
    let offset = (page - 1) * size;
    let search_pattern = format!("%{}%", title); // 前后模糊匹配

    let sql = "
    SELECT * FROM post
    WHERE post_title ILIKE $1
    ORDER BY post_create_time DESC
    LIMIT $2 OFFSET $3";

    let posts = sqlx::query_as::<_, Post>(sql)
        .bind(search_pattern)
        .bind(size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .unwrap();

    Ok(posts)
}

/// 增加文章点赞数
pub async fn increment_likes(pool: &PgPool, post_id: i64) -> SqlResult<bool> {
    let sql = "
    UPDATE post SET
        post_likes = post_likes + 1,
        post_update_time = $1
    WHERE post_id = $2";

    let now = Utc::now();
    let rows_affected = sqlx::query(sql)
        .bind(now)
        .bind(post_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

/// 增加文章收藏数
pub async fn increment_favorites(pool: &PgPool, post_id: i64) -> SqlResult<bool> {
    let sql = "
    UPDATE post SET
        post_favorites = post_favorites + 1,
        post_update_time = $1
    WHERE post_id = $2";

    let now = Utc::now();
    let rows_affected = sqlx::query(sql)
        .bind(now)
        .bind(post_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}
