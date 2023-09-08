use std::sync::Arc;

use crate::app::models::ticket::{Ticket, Status};
use crate::app::models::point::Point;
use crate::ticket_proto::{GetTicketRequest, GetTicketResponse,VerifyTicketRequest,VerifyTicketResponse};
use crate::app::types::error::Error;
use chrono::Utc;
use sqlx::{Postgres, Row};

pub async fn get_ticket(db_pool:&sqlx::Pool<Postgres>,data:GetTicketRequest)->Result<GetTicketResponse,Error>{
    let ticket = Ticket::new(data.user_id, Point::from(data.point), data.ip, data.agent);
    sqlx::query("INSERT INTO tickets(ticket_id,user_id,point,status,ip,agent,expire_at) VALUES ($1,$2,$3,$4,$5,$6,$7)")
    .bind(ticket.ticket_id)
    .bind(ticket.user_id)
    .bind(ticket.point.to_string())
    .bind(ticket.status.to_string())
    .bind(ticket.ip)
    .bind(ticket.agent)
    .bind(ticket.expire_at)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    Ok(GetTicketResponse{ticket_id:ticket.ticket_id})
}
pub async fn verify(db_pool:&sqlx::Pool<Postgres>,data:VerifyTicketRequest)->Result<VerifyTicketResponse,Error>{
    let agent = Arc::new(data.agent);
    let ip = Arc::new(data.ip);

    let row = sqlx::query("SELECT user_id,point,status,expire_at FROM tickets WHERE ticket_id = $1 AND ip = $2 AND agent = $3")
    .bind(data.ticket_id)
    .bind(ip.clone().as_ref())
    .bind(agent.clone().as_ref())
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::InternalError(e.to_string()))?;

    if row.is_none(){
        return Err(Error::NotFoundError("not found #404".to_owned()))
    }
    let row = row.unwrap();
    let status = Status::from(row.get::<String,_>("status"));
    if !status.validate(){
        return Err(Error::PermissionDeniedError("invalid ticket #400".to_owned()));
    }
    let user_id = row.get::<i32,_>("user_id");
    let point = row.get::<String,_>("point");
    let expire_at = row.get::<chrono::DateTime<Utc>,_>("expire_at");
    if expire_at < chrono::Utc::now(){
        return Err(Error::PermissionDeniedError("ticket expired #400".to_owned()))
    }
    Ok(VerifyTicketResponse{
        point:Point::from(point).to_i32(),
        user_id,
        agent:agent.as_ref().to_owned(),
        ip:ip.as_ref().to_owned(),
    })
}