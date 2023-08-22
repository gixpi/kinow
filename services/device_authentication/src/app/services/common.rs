use sqlx::Postgres;

use crate::app::types::error::Error;
use crate::app::models::token::Token;

pub async fn device_exists_by_serial_and_user_id_pg(db_pool:&sqlx::Pool<Postgres>,serial_code:&String,user_id:&i32)->Result<(),Error>{
    let data = sqlx::query("SELECT device_id FROM devices WHERE serial_code = $1 AND user_id = $2")
    .bind(serial_code)
    .bind(user_id)
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::InternalError(e.to_string()))?;

    if data.is_some(){
        return Err(Error::ServiceError(format!("serial code already exists")))
    }
    Ok(())
}

pub async fn create_token(db_pool:&sqlx::Pool<Postgres>,token_data:&Token)->Result<(),Error>{
    sqlx::query("INSERT INTO tokens (access_token,refresh_token,device_id,token_status,ip,created_at,access_token_expire_at,refresh_token_expire_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)")
    .bind(token_data.access_token.clone())
    .bind(token_data.refresh_token.clone())
    .bind(token_data.device_id)
    .bind(token_data.status.to_string())
    .bind(token_data.ip.clone())
    .bind(token_data.created_at)
    .bind(token_data.access_token_expire_at)
    .bind(token_data.refresh_token_expire_at)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    return Ok(())
}