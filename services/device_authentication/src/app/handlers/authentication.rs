use std::sync::Arc;

use sqlx::Postgres;
use tonic::{Request, Response, Status};
use crate::app::services;
use crate::authentication_proto::authentication_service_server::AuthenticationService;
use crate::authentication_proto::{TokenInfo,SignupRequest, SigninRequest, SignupResponse};

pub struct AuthenticationHandler{
    pub postgres_db:Arc<sqlx::Pool<Postgres>>,
    pub token_life_expiry:i32
}

impl AuthenticationHandler{
    pub fn new(postgres_db:Arc<sqlx::Pool<Postgres>>,token_life_expiry:i32)->Self{
        Self { 
            postgres_db,
            token_life_expiry
        }
    }
}

#[tonic::async_trait]
impl AuthenticationService for AuthenticationHandler{
    async fn signup(&self,request:Request<SignupRequest>)->Result<Response<SignupResponse>,Status>{
        let res = services::authentication::signup(&self.postgres_db, request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
    async fn signin(&self,request:Request<SigninRequest>)->Result<Response<TokenInfo>,Status>{
        let res = services::authentication::signin(&self.postgres_db, request.into_inner(),self.token_life_expiry).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
}