use crate::repository;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserGroup {
    pub id: i64,
    pub yug_code: String,
    pub yug_name: String,
    pub yug_memo: String,
    pub yug_active: bool,
}

pub async fn add_usergroup(yug: UserGroup) -> sqlx::Result<UserGroup> {
    let state = repository::get_state().await?;
    let usergroup = sqlx::query_as::<_, UserGroup>(
        "INSERT INTO yug_usergroups (yug_code, yug_name, yug_memo, yug_active) VALUES ($1, $2, $3, $4) RETURNING id, yug_code, yug_name, yug_memo, yug_active"
    )
    .bind(yug.yug_code)
    .bind(yug.yug_name)
    .bind(yug.yug_memo)
    .bind(yug.yug_active)
    .fetch_one(&state.pool)
    .await?;
    Ok(usergroup)
}

pub async fn set_usergroup(yug: UserGroup) -> sqlx::Result<UserGroup> {
    let state = repository::get_state().await?;
    let usergroup = sqlx::query_as::<_, UserGroup>(
        "UPDATE yug_usergroups SET yug_name = $2, yug_memo = $3, yug_active = $4 WHERE yug_code = $1 RETURNING id, yug_code, yug_name, yug_memo, yug_active"
    )
    .bind(yug.yug_code)
    .bind(yug.yug_name)
    .bind(yug.yug_memo)
    .bind(yug.yug_active)
    .fetch_one(&state.pool)
    .await?;
    Ok(usergroup)
}

pub async fn delete_usergroup_by_id(yug_id: i64) -> sqlx::Result<bool> {
    let state = repository::get_state().await?;
    let _usergroup = sqlx::query_as::<_, UserGroup>(
        "DELETE FROM yug_usergroups WHERE id = $1 RETURNING id, yug_code, yug_name, yug_memo, yug_active"
    )
    .bind(yug_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(true)
}

pub async fn delete_usergroup_by_code(yug_code: &str) -> sqlx::Result<bool> {
    let state = repository::get_state().await?;
    let _usergroup = sqlx::query_as::<_, UserGroup>(
        "DELETE FROM yug_usergroups WHERE yug_code = $1 RETURNING id, yug_code, yug_name, yug_memo, yug_active"
    )
    .bind(yug_code)
    .fetch_one(&state.pool)
    .await?;
    Ok(true)
}

pub async fn get_usergroup_by_code(yug_code: &str) -> sqlx::Result<UserGroup> {
    let state = repository::get_state().await?;
    let usergroup = sqlx::query_as::<_, UserGroup>(
        "SELECT id, yug_code, yug_name, yug_memo, yug_active FROM yug_usergroups WHERE yug_code = $1"
    )
    .bind(yug_code)
    .fetch_one(&state.pool)
    .await?;
    Ok(usergroup)
}
pub async fn get_usergroup_by_id(yug_id: i64) -> sqlx::Result<UserGroup> {
    let state = repository::get_state().await?;
    let usergroup = sqlx::query_as::<_, UserGroup>(
        "SELECT id, yug_code, yug_name, yug_memo, yug_active FROM yug_usergroups WHERE id = $1"
    )
    .bind(yug_id)
    .fetch_one(&state.pool)
    .await?;
    Ok(usergroup)
}

pub async fn get_usergroup_list(filter: &str, page_nav: repository::Navigation) -> sqlx::Result<Vec<UserGroup>> {
    let state = repository::get_state().await?;
    let usergroups = sqlx::query_as::<_, UserGroup>(
        "SELECT id, yug_code, yug_name, yug_memo, yug_active FROM yug_usergroups WHERE yug_code LIKE $1 LIMIT $2 OFFSET $3"
    )
    .bind(format!("%{filter}%"))
    .bind(page_nav.page_length)
    .bind((page_nav.page_number - 1) * page_nav.page_length)
    .fetch_all(&state.pool)
    .await?;
    Ok(usergroups)
}

pub async fn get_usergroup_count(filter: &str) -> sqlx::Result<i64> {
    let state = repository::get_state().await?;
    let count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM yug_usergroups WHERE yug_code LIKE $1"
    )
    .bind(format!("%{filter}%"))
    .fetch_one(&state.pool)
    .await?;
    Ok(count)
}

