use crate::{
    model::user::UserClaim,
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};
use axum::extract::Path;

// 删除非遗项目
pub async fn delete(user_claim: UserClaim, Path(ich_id): Path<i64>) -> AppResult<()> {
    let pool = database_connect();

    // 先检查是否存在
    if !sql::ich::exists(&pool, ich_id).await? {
        return Err(AppError::IchNotFound);
    }

    let ich = sql::ich::get_by_id(pool, ich_id).await?;

    if ich.ich_uploader != user_claim.data.user_id || user_claim.data.user_identity != 0 {
        return Err(AppError::InvalidToken);
    }
    // 执行删除
    let deleted = sql::ich::delete(&pool, ich_id).await?;

    if !deleted {
        return Err(AppError::IchDeleteFailed);
    }

    Ok(AppResponse::success(None))
}
