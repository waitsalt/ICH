use axum::extract::Query;

use crate::{
    model::post::{Post, PostSearchParam},
    sql,
    util::{AppResult, database::database_connect, response::AppResponse},
};

/// 搜索文章
pub async fn search(
    Query(search_param): Query<PostSearchParam>, // 假设有搜索参数结构体
) -> AppResult<Vec<Post>> {
    let pool = database_connect();

    let posts = sql::post::search_by_title(
        &pool,
        &search_param.keyword,
        search_param.page,
        search_param.size,
    )
    .await
    .unwrap();

    Ok(AppResponse::success(Some(posts)))
}
