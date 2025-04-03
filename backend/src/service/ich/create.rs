use crate::{
    model::{ich::IchCreatePayload, user::UserClaim}, // 假设有这个模型
    sql,
    util::{AppResult, database::database_connect, error::AppError, response::AppResponse},
};
use axum::Json;

pub async fn create(
    user_claim: UserClaim,
    Json(ich_payload): Json<IchCreatePayload>,
) -> AppResult<i64> {
    let pool = database_connect();

    // 检查项目编号是否已存在
    if sql::ich::exists_by_code(&pool, &ich_payload.ich_code).await? {
        return Err(AppError::IchCodeAlreadyExists);
    }

    // 创建非遗项目记录
    let ich_id = sql::ich::create(
        &pool,
        &ich_payload.ich_code,
        &ich_payload.ich_name,
        &ich_payload.ich_publish,
        &ich_payload.ich_category,
        &ich_payload.ich_location,
        &ich_payload.ich_apply_location,
        &ich_payload.ich_protect_unit,
        &ich_payload.ich_content,
        user_claim.data.user_id, // 通常从当前登录用户获取
    )
    .await?;

    Ok(AppResponse::success(Some(ich_id)))
}
