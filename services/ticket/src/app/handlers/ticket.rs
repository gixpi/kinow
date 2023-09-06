use std::sync::Arc;
use sqlx::Postgres;
use tonic::{Request, Response, Status};
use crate::ticket_proto::ticket_service_server::TicketService;
use crate::ticket_proto::{GetTicketRequest,GetTicketResponse,Empty,VerifyTicketRequest};



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
        todo!("Get Ticket")
    }
    async fn verify(&self,request:Request<VerifyTicketRequest>)->Result<Response<Empty>,Status>{
        todo!("Verify")
    }
}