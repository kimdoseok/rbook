use crate::repository;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserGroup {
    pub id: i64,
    pub yug_code: String,
    pub yug_name: String,
    pub yug_memo: String,
    pub yug_active: bool,
}

impl From<crate::serv_system::proto::UserGroupAddRequest> for UserGroup {
    fn from(req: crate::serv_system::proto::UserGroupAddRequest) -> Self {
        Self {
            id: 0,
            yug_code: req.yug_code,
            yug_name: req.yug_name,
            yug_memo: req.yug_memo,
            yug_active: req.yug_active,
        }
    }
}

impl From<crate::serv_system::proto::UserGroupSetRequest> for UserGroup {
    fn from(req: crate::serv_system::proto::UserGroupSetRequest) -> Self {
        Self {
            id: req.id,
            yug_code: req.yug_code,
            yug_name: req.yug_name,
            yug_memo: req.yug_memo,
            yug_active: req.yug_active,
        }
    }
}

impl From<crate::serv_system::proto::UserGroupResponse> for UserGroup {
    fn from(resp: crate::serv_system::proto::UserGroupResponse) -> Self {
        UserGroup {
            id: resp.id,
            yug_code: resp.yug_code,
            yug_name: resp.yug_name,
            yug_memo: resp.yug_memo,
            yug_active: resp.yug_active,
        }
    }
}

impl From<UserGroup> for crate::serv_system::proto::UserGroupAddRequest {
    fn from(yug: UserGroup) -> Self {
        crate::serv_system::proto::UserGroupAddRequest {
            yug_code: yug.yug_code,
            yug_name: yug.yug_name,
            yug_memo: yug.yug_memo,
            yug_active: yug.yug_active,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serv_system::proto::UserGroupAddRequest;

    #[test]
    fn converts_add_request_to_user_group_model() {
        let req = UserGroupAddRequest {
            yug_code: "TEST".into(),
            yug_name: "Test Group".into(),
            yug_memo: "memo".into(),
            yug_active: true,
        };

        let model: UserGroup = req.into();
        assert_eq!(model.yug_code, "TEST");
        assert_eq!(model.yug_name, "Test Group");
        assert_eq!(model.yug_memo, "memo");
        assert!(model.yug_active);
    }
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

