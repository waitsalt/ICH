use crate::{
    model::ich::{Ich, IchQueryParam},
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};
use axum::extract::Query;

// 模糊查询非遗项目
pub async fn search(Query(params): Query<IchQueryParam>) -> AppResult<Vec<Ich>> {
    let pool = database_connect();

    let ich_list =
        sql::ich::search_by_name(&pool, &params.keyword, params.page, params.size).await?;

    Ok(AppResponse::success(Some(ich_list)))
}
