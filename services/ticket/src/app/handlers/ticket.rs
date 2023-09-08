use std::sync::Arc;
use sqlx::Postgres;
use tonic::{Request, Response, Status};
use crate::app::services;
use crate::ticket_proto::ticket_service_server::TicketService;
use crate::ticket_proto::{GetTicketRequest,GetTicketResponse,VerifyTicketRequest,VerifyTicketResponse};



pub struct TicketHandler{
    pub postgres_db:Arc<sqlx::Pool<Postgres>>,
}

impl TicketHandler{
    pub fn new(postgres_db:Arc<sqlx::Pool<Postgres>>)->Self{
        Self { postgres_db }
    }
}

#[tonic::async_trait]
impl TicketService for TicketHandler{
    async fn get_ticket(&self,request:Request<GetTicketRequest>)->Result<Response<GetTicketResponse>,Status>{
        let res = services::ticket::get_ticket(&self.postgres_db, request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
    async fn verify(&self,request:Request<VerifyTicketRequest>)->Result<Response<VerifyTicketResponse>,Status>{
        let res = services::ticket::verify(&self.postgres_db, request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
}