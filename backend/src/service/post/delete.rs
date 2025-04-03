use axum::{Json, extract::Path};

use crate::{
    model::{post::PostCreatePayload, user::UserClaim},
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};

pub async fn delete(user_claim: UserClaim, Path(post_id): Path<i64>) -> AppResult<bool> {
    let pool = database_connect();

    // 验证文章是否存在且属于当前用户
    let post = sql::post::get_by_id(&pool, post_id).await?;
    if post.user_id != user_claim.data.user_id {
        return Err(AppError::PermissionDenied);
    }

    // 删除文章
    let success = sql::post::delete(&pool, post_id).await?;

    Ok(AppResponse::success(Some(success)))
}
