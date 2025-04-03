use crate::{
    model::ich::Ich,
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};
use axum::extract::Path;

// 模糊查询非遗项目
pub async fn info(Path(ich_id): Path<i64>) -> AppResult<Ich> {
    let pool = database_connect();

    let ich = sql::ich::get_by_id(&pool, ich_id).await?;

    Ok(AppResponse::success(Some(ich)))
}
