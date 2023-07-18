use std::sync::Arc;

use bb8_redis::RedisConnectionManager;
use sqlx::Postgres;
use tonic::{Request, Response, Status};
use crate::app::services;
use crate::authentication_proto::authentication_service_server::AuthenticationService;
use crate::authentication_proto::{SignupRequest,TokenInfo,OptionalResponse};
pub struct AuthenticationHandler{
    pub postgres_db:Arc<sqlx::Pool<Postgres>>,
    pub redis_db:Arc<bb8::Pool<RedisConnectionManager>>,
    token_life_exitry:i32
}

impl AuthenticationHandler{
    pub fn new(postgres_db:Arc<sqlx::Pool<Postgres>>,redis_db:Arc<bb8::Pool<RedisConnectionManager>>,token_life_exitry:i32)->Self{
        Self { 
            postgres_db, 
            redis_db,
            token_life_exitry
        }
    }
}

#[tonic::async_trait]
impl AuthenticationService for AuthenticationHandler{
    async fn signup(&self,request:Request<SignupRequest>)->Result<Response<OptionalResponse>,Status>{
        let res = services::authentication::signup(&self.postgres_db.as_ref(), &self.redis_db.as_ref(), request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }

}

