use crate::{
    model::{post::PostUpdatePayload, user::UserClaim},
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};
use axum::{Json, extract::Path};

// 更新文章
pub async fn update(
    user_claim: UserClaim,
    Path(post_id): Path<i64>,
    Json(payload): Json<PostUpdatePayload>,
) -> AppResult<()> {
    let pool = database_connect();

    // 1. 检查项目是否存在
    if !sql::post::exists(&pool, post_id).await? {
        return Err(AppError::IchNotFound);
    }

    // 2. 获取项目并验证上传者权限
    let post = sql::post::get_by_id(&pool, post_id).await?;
    if post.user_id != user_claim.data.user_id {
        return Err(AppError::PermissionDenied);
    }

    // 4. 执行更新
    let updated = sql::post::update(
        &pool,
        post.post_id,
        &payload.post_title,
        &payload.post_content,
        payload.post_media_urls,
        payload.post_type,
    )
    .await?;

    if !updated {
        return Err(AppError::IchUpdateFailed);
    }

    Ok(AppResponse::success(None))
}
