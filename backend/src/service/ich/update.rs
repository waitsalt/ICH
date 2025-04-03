use crate::{
    model::{ich::IchUpdatePayload, user::UserClaim},
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};
use axum::{Json, extract::Path};

// 更新非遗项目
pub async fn update(
    user_claim: UserClaim,
    Path(ich_id): Path<i64>,
    Json(payload): Json<IchUpdatePayload>,
) -> AppResult<()> {
    let pool = database_connect();

    // 1. 检查项目是否存在
    if !sql::ich::exists(&pool, ich_id).await? {
        return Err(AppError::IchNotFound);
    }

    // 2. 获取项目并验证上传者权限
    let ich = sql::ich::get_by_id(&pool, ich_id).await?;
    if ich.ich_uploader != user_claim.data.user_id || user_claim.data.user_identity != 0 {
        return Err(AppError::PermissionDenied);
    }

    // 3. 检查项目编号是否冲突（如果修改了编号）
    {
        let new_code = &payload.ich_code;
        if new_code != &ich.ich_code && sql::ich::exists_by_code(&pool, new_code).await? {
            return Err(AppError::IchCodeConflict);
        }
    }

    // 4. 执行更新
    let updated = sql::ich::update(
        &pool,
        &ich_id,
        &payload.ich_code,
        &payload.ich_name,
        &payload.ich_publish,
        &payload.ich_category,
        &payload.ich_location,
        &payload.ich_apply_location,
        &payload.ich_protect_unit,
        &payload.ich_content,
    )
    .await?;

    if !updated {
        return Err(AppError::IchUpdateFailed);
    }

    Ok(AppResponse::success(None))
}
