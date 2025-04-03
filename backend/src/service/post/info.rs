use crate::{
    model::post::Post,
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};
use axum::extract::Path;

// 模糊查询非遗项目
pub async fn info(Path(post_id): Path<i64>) -> AppResult<Post> {
    let pool = database_connect();

    let post = sql::post::get_by_id(&pool, post_id).await?;

    Ok(AppResponse::success(Some(post)))
}
