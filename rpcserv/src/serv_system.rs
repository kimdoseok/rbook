use tonic::{Request, Response, Status};

use crate::{repo_system, repository};

pub mod proto {
    tonic::include_proto!("system");
}

pub use proto::user_group_service_server;

use proto::{
    UserGroupAddRequest, UserGroupCountResponse, UserGroupDeleteResponse, UserGroupListRequest,
    UserGroupListResponse, UserGroupRequest, UserGroupResponse, UserGroupSetRequest,
};

fn map_repo_error(err: sqlx::Error, context: &str) -> Status {
    match err {
        sqlx::Error::RowNotFound => Status::not_found(format!("{context}: no user group found")),
        _ => Status::internal(format!("{context}: {err}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_row_not_found_to_not_found_status() {
        let status = map_repo_error(sqlx::Error::RowNotFound, "get_user_group_by_code");
        assert_eq!(status.code(), tonic::Code::NotFound);
        assert!(status.message().contains("get_user_group_by_code"));
    }
}

#[derive(Debug, Default)]
pub struct MyUserGroup;

impl From<repo_system::UserGroup> for UserGroupResponse {
    fn from(yug: repo_system::UserGroup) -> Self {
        UserGroupResponse {
            id: yug.id,
            yug_code: yug.yug_code,
            yug_name: yug.yug_name,
            yug_memo: yug.yug_memo,
            yug_active: yug.yug_active,
        }
    }
}

#[tonic::async_trait]
impl user_group_service_server::UserGroupService for MyUserGroup {
    async fn get_user_group_by_id(
        &self,
        request: Request<UserGroupRequest>,
    ) -> Result<Response<UserGroupResponse>, Status> {
        let req = request.into_inner();
        let yug = repo_system::get_usergroup_by_id(req.id)
            .await
            .map_err(|e| map_repo_error(e, "get_user_group_by_id"))?;

        Ok(Response::new(yug.into()))
    }

    async fn get_user_group_by_code(
        &self,
        request: Request<UserGroupRequest>,
    ) -> Result<Response<UserGroupResponse>, Status> {
        let req = request.into_inner();
        //println!("get_user_group_by_code: req.yug_code = {}", req.yug_code);
        let yug = repo_system::get_usergroup_by_code(&req.yug_code)
            .await
            .map_err(|e| map_repo_error(e, "get_user_group_by_code"))?;

        Ok(Response::new(yug.into()))
    }

    async fn get_user_group_list(
        &self,
        _request: Request<UserGroupListRequest>,
    ) -> Result<Response<UserGroupListResponse>, Status> {
        let req = _request.into_inner();
        let page_nav = repository::Navigation {
            page_length: 100,
            page_number: 1,
            page_current: 1,
        };

        let yugs = repo_system::get_usergroup_list(&req.filter, page_nav)
            .await
            .map_err(|e| map_repo_error(e, "get_user_group_list"))?;
        let yugs: Vec<UserGroupResponse> = yugs.into_iter().map(|yug| yug.into()).collect();

        let grpc_message = UserGroupListResponse { usergroups: yugs };
        Ok(Response::new(grpc_message))
    }

    async fn set_user_group(
        &self,
        request: Request<UserGroupSetRequest>,
    ) -> Result<Response<UserGroupResponse>, Status> {
        let req = request.into_inner();

        let yug = repo_system::set_usergroup(req.into())
            .await
            .map_err(|e| map_repo_error(e, "set_user_group"))?;

        Ok(Response::new(yug.into()))
    }

    async fn add_user_group(
        &self,
        request: Request<UserGroupAddRequest>,
    ) -> Result<Response<UserGroupResponse>, Status> {
        let req = request.into_inner();
        let yug = repo_system::add_usergroup(req.into())
            .await
            .map_err(|e| map_repo_error(e, "add_user_group"))?;

        Ok(Response::new(yug.into()))
    }

    async fn get_user_group_count(
        &self,
        _request: Request<UserGroupListRequest>,
    ) -> Result<Response<UserGroupCountResponse>, Status> {
        let req = _request.into_inner();
        let count = repo_system::get_usergroup_count(&req.filter)
            .await
            .map_err(|e| map_repo_error(e, "get_user_group_count"))?;

        Ok(Response::new(UserGroupCountResponse { count }))
    }

    async fn delete_user_group_by_id(
        &self,
        _request: Request<UserGroupRequest>,
    ) -> Result<Response<UserGroupDeleteResponse>, Status> {
        let req = _request.into_inner();
        let _ = repo_system::delete_usergroup_by_id(req.id)
            .await
            .map_err(|e| map_repo_error(e, "delete_user_group_by_id"));
        Ok(Response::new(UserGroupDeleteResponse { success: true }))
    }
    async fn delete_user_group_by_code(
        &self,
        _request: Request<UserGroupRequest>,
    ) -> Result<Response<UserGroupDeleteResponse>, Status> {
        let req = _request.into_inner();
        let _ = repo_system::delete_usergroup_by_code(&req.yug_code)
            .await
            .map_err(|e| map_repo_error(e, "delete_user_group_by_code"));
        Ok(Response::new(UserGroupDeleteResponse { success: true }))
    }
}
