use sqlx::postgres::PgPool;

use crate::model::ich::Ich;
use crate::util::error::AppError;

use super::SqlResult;

/// 创建非遗项目记录
pub async fn create(
    pool: &PgPool,
    ich_code: &str,
    ich_name: &str,
    ich_publish: &str,
    ich_category: &str,
    ich_location: &str,
    ich_apply_location: &str,
    ich_protect_unit: &str,
    ich_content: &str,
    ich_uploader: i64,
) -> SqlResult<i64> {
    let sql = "
    INSERT INTO ich (
        ich_code, ich_name, ich_publish, ich_category,
        ich_location, ich_apply_location, ich_protect_unit,
        ich_content, ich_uploader
    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING ich_id";

    let ich_id = sqlx::query_scalar(sql)
        .bind(ich_code)
        .bind(ich_name)
        .bind(ich_publish)
        .bind(ich_category)
        .bind(ich_location)
        .bind(ich_apply_location)
        .bind(ich_protect_unit)
        .bind(ich_content)
        .bind(ich_uploader)
        .fetch_one(pool)
        .await?;

    Ok(ich_id)
}

/// 根据ID查询非遗项目
pub async fn get_by_id(pool: &PgPool, ich_id: i64) -> SqlResult<Ich> {
    let sql = "SELECT * FROM ich WHERE ich_id = $1";

    let ich_opt = sqlx::query_as::<_, Ich>(sql)
        .bind(ich_id)
        .fetch_optional(pool)
        .await?;

    match ich_opt {
        Some(ich) => Ok(ich),
        None => Err(AppError::IchNotFound),
    }
}

/// 更新非遗项目信息
pub async fn update(
    pool: &PgPool,
    ich_id: &i64,
    ich_code: &str,
    ich_name: &str,
    ich_publish: &str,
    ich_category: &str,
    ich_location: &str,
    ich_apply_location: &str,
    ich_protect_unit: &str,
    ich_content: &str,
) -> SqlResult<bool> {
    let sql = "
    UPDATE ich SET
        ich_code = $1,
        ich_name = $2,
        ich_publish = $3,
        ich_category = $4,
        ich_location = $5,
        ich_apply_location = $6,
        ich_protect_unit = $7,
        ich_content = $8
    WHERE ich_id = $9";

    let rows_affected = sqlx::query(sql)
        .bind(ich_code)
        .bind(ich_name)
        .bind(ich_publish)
        .bind(ich_category)
        .bind(ich_location)
        .bind(ich_apply_location)
        .bind(ich_protect_unit)
        .bind(ich_content)
        .bind(ich_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

/// 删除非遗项目
pub async fn delete(pool: &PgPool, ich_id: i64) -> SqlResult<bool> {
    let sql = "DELETE FROM ich WHERE ich_id = $1";

    let rows_affected = sqlx::query(sql)
        .bind(ich_id)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_affected > 0)
}

/// 检查非遗项目是否存在
pub async fn exists(pool: &PgPool, ich_id: i64) -> SqlResult<bool> {
    let sql = "SELECT EXISTS(SELECT 1 FROM ich WHERE ich_id = $1)";

    let exists = sqlx::query_scalar(sql).bind(ich_id).fetch_one(pool).await?;

    Ok(exists)
}

/// 根据项目编号检查是否存在
pub async fn exists_by_code(pool: &PgPool, ich_code: &str) -> SqlResult<bool> {
    let sql = "SELECT EXISTS(SELECT 1 FROM ich WHERE ich_code = $1)";

    let exists = sqlx::query_scalar(sql)
        .bind(ich_code)
        .fetch_one(pool)
        .await?;

    Ok(exists)
}

// 根据名称模糊查询（带分页）
pub async fn search_by_name(
    pool: &PgPool,
    keyword: &str,
    page: u32,
    size: u32,
) -> SqlResult<Vec<Ich>> {
    let offset = (page - 1) * size;
    let search_pattern = format!("%{}%", keyword); // 前后模糊匹配

    let sql = "
    SELECT * FROM ich
    WHERE ich_name ILIKE $1
    ORDER BY ich_id
    LIMIT $2 OFFSET $3";

    let ich_list = sqlx::query_as::<_, Ich>(sql)
        .bind(search_pattern)
        .bind(size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

    Ok(ich_list)
}
