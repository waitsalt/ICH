use sqlx::{Pool, Postgres};

use crate::{model::user::User, sql::SqlResult, util::error::AppError};

pub async fn user_info_get_by_id(pool: &Pool<Postgres>, id: &i64) -> SqlResult<User> {
    let sql = "
    select
        *
    from
        \"user\"
    where
        user_id = $1";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(id)
        .fetch_optional(pool)
        .await
        .unwrap();
    match res {
        Some(user) => {
            return Ok(user);
        }
        None => {
            return Err(AppError::UserNotExist);
        }
    }
}

pub async fn user_info_get_by_name(pool: &Pool<Postgres>, name: &str) -> SqlResult<User> {
    let sql = "
    select
        *
    from
        \"user\"
    where
        user_name = $1";
    let res: Option<User> = sqlx::query_as(sql)
        .bind(name)
        .fetch_optional(pool)
        .await
        .unwrap();
    match res {
        Some(user) => {
            return Ok(user);
        }
        None => {
            return Err(AppError::UserNotExist);
        }
    }
}

pub async fn user_search_by_name(
    pool: &Pool<Postgres>,
    name: &str,
    page: u32,
    size: u32,
) -> SqlResult<Vec<User>> {
    let offset = (page - 1) * size;

    let sql = "
    SELECT * FROM user
    WHERE user_name ILIKE $1
    ORDER BY post_create_time DESC
    LIMIT $2 OFFSET $3";

    let posts = sqlx::query_as::<_, User>(sql)
        .bind(name)
        .bind(size as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

    Ok(posts)
}

pub async fn user_name_is_exist(pool: &Pool<Postgres>, name: &str) -> SqlResult<()> {
    let sql = "
    select
        user_id
    from
        \"user\"
    where
        user_name = $1";
    let affected_row = sqlx::query(sql)
        .bind(name)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserNameExist);
    }
    Ok(())
}

pub async fn user_email_is_exist(pool: &Pool<Postgres>, email: &str) -> SqlResult<()> {
    let sql = "
        select
            user_id
        from
            \"user\"
        where
            user_email = $1";
    let affected_row = sqlx::query(sql)
        .bind(email)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserEmailExist);
    }
    Ok(())
}

pub async fn user_phone_is_exist(pool: &Pool<Postgres>, phone: &str) -> SqlResult<()> {
    let sql = "
        select
            user_id
        from
            \"user\"
        where
            user_phone = $1";
    let affected_row = sqlx::query(sql)
        .bind(phone)
        .execute(pool)
        .await
        .unwrap()
        .rows_affected();
    if affected_row != 0 {
        return Err(AppError::UserEmailExist);
    }
    Ok(())
}

pub async fn user_create(
    pool: &Pool<Postgres>,
    name: &str,
    password: &str,
    email: &str,
    avatar_url: &str,
) -> SqlResult<()> {
    let sql = "
    insert into
        \"user\" (user_name, user_password, user_email, user_avatar_url)
    values
        ($1,$2,$3,$4)";
    let _affected_row = sqlx::query(sql)
        .bind(name)
        .bind(password)
        .bind(email)
        .bind(avatar_url)
        .execute(pool)
        .await
        .unwrap()
        // .map_err(|_| AppError::UserCreateFailure)?
        .rows_affected();
    Ok(())
}

pub async fn user_delete(pool: &Pool<Postgres>, id: &i64) -> SqlResult<()> {
    let sql = "
    upadte
        \"user\"
    set
        user_status = 2
    where
        user_id = $1";
    let _ = sqlx::query(sql).bind(id).execute(pool).await.unwrap();
    Ok(())
}

pub async fn user_all(pool: &Pool<Postgres>) -> SqlResult<Vec<User>> {
    let sql = "
    select
        *
    from
        \"user\"";

    let users: Vec<User> = sqlx::query_as(sql).fetch_all(pool).await.unwrap();
    Ok(users)
}

pub async fn update_base_info(
    pool: &Pool<Postgres>,
    user_id: &i32,
    user_avatar_url: &str,
    user_name: &str,
) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_avatar_url = $1
    and
        user_name = $2
    where
        user_id = $3";
    let _ = sqlx::query(sql)
        .bind(user_avatar_url)
        .bind(user_name)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_avatar_url(pool: &Pool<Postgres>, id: &i32, avatar_url: &str) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_avatar_url = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(avatar_url)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_email(pool: &Pool<Postgres>, id: &i32, email: &str) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_email = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(email)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}

pub async fn update_password(pool: &Pool<Postgres>, id: &i32, password: &str) -> SqlResult<()> {
    let sql = "
    update
        \"user\"
    set
        user_password = $1
    where
        user_id = $2";
    let _ = sqlx::query(sql)
        .bind(password)
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}
