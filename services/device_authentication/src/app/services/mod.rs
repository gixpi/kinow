use std::sync::Arc;

use sqlx::Postgres;
use tonic::{Request, Response, Status};
use crate::authentication_proto::authentication_serivce_server::AuthenticationSerivce;
use crate::authentication_proto::{Empty,SignupRequest, SigninRequest};

pub struct AuthenticationHandler{
    pub postgres_db:Arc<sqlx::Pool<Postgres>>,
}

impl AuthenticationHandler{
    pub fn new(postgres_db:Arc<sqlx::Pool<Postgres>>)->Self{
        Self { postgres_db }
    }
}

#[tonic::async_trait]
impl AuthenticationSerivce for AuthenticationHandler{
    async fn signup(&self,request:Request<SignupRequest>)->Result<Response<Empty>,Status>{
        todo!("signup")
    }
    async fn signin(&self,request:Request<SigninRequest>)->Result<Response<Empty>,Status>{
        todo!("signin")
    }
}