use sqlx::Postgres;
use crate::app::models::ticket::Ticket;
use crate::app::models::point::Point;
use crate::ticket_proto::{GetTicketRequest, GetTicketResponse};
use crate::app::types::error::Error;

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

    Err(Error::DBPoolError("".to_owned()))
}