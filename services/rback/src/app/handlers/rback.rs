use std::sync::Arc;

use sqlx::Postgres;
use tonic::{Request, Status, Response};
use crate::rback_proto::r_back_service_server::RBackService;
use crate::rback_proto::{
    Empty,
    Roles,
    Permissions, 
    VerifyUserPermissionRequest, 
    GetUserRolesRequest, 
    GetRolePermissionsRequest
    };
pub struct RBackHandler{
    pub postgres_db:Arc<sqlx::Pool<Postgres>>,
}

impl RBackHandler{
    pub fn new(postgres_db:Arc<sqlx::Pool<Postgres>>)->Self{
        Self { postgres_db }
    }
}

#[tonic::async_trait]
impl RBackService for RBackHandler{
    async fn verify_user_permission(&self,request:Request<VerifyUserPermissionRequest>)->Result<Response<Empty>,Status>{
        todo!("Add Role")
    }
    async fn get_all_roles(&self,request:Request<Empty>)->Result<Response<Roles>,Status>{
        todo!("Get All Roles")
    }
    async fn get_all_permissions(&self,request:Request<Empty>)->Result<Response<Permissions>,Status>{
        todo!("Get All Permissions")
    }
    async fn get_role_permissions(&self,request:Request<GetRolePermissionsRequest>)->Result<Response<Permissions>,Status>{
        todo!("Get Role Permissions")
    }
    async fn get_user_roles(&self,request:Request<GetUserRolesRequest>)->Result<Response<Roles>,Status>{
        todo!("Get User Roles")
    }
}