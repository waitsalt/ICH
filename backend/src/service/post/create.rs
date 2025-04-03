use axum::Json;

use crate::{
    model::{post::PostCreatePayload, user::UserClaim},
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};

/// 创建文章
pub async fn create(
    user_claim: UserClaim,
    Json(post_payload): Json<PostCreatePayload>,
) -> AppResult<i64> {
    let pool = database_connect();

    // 创建文章记录
    let post_id = sql::post::create(
        &pool,
        user_claim.data.user_id, // 使用当前登录用户的ID
        post_payload.ich_id,
        &post_payload.post_title,
        &post_payload.post_content,
        post_payload.post_media_urls,
        post_payload.post_type,
    )
    .await?;

    Ok(AppResponse::success(Some(post_id)))
}
