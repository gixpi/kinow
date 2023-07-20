use crate::app::models::token::Token;
use crate::app::types::error::Error;
use bb8_redis::RedisConnectionManager;
use bb8_redis::redis::AsyncCommands;
use sqlx::Postgres;

pub async fn user_exists_by_phone_pg(db_pool:&sqlx::Pool<Postgres>,phone_number:&String)->Result<(),Error>{
    let data = sqlx::query("SELECT user_id FROM users WHERE phone_number = $1").bind(phone_number).fetch_optional(db_pool)
    .await
    .map_err(|e| Error::InternalError(e.to_string()))?;

    if data.is_some(){
        return Err(Error::ServiceError(format!("phone number already exists")))
    }
    Ok(())
}

pub async fn phone_number_exists_rd(db_pool:&bb8::Pool<RedisConnectionManager>,phone_number:&String)->Result<(),Error>{
    let mut rd_db_pool =  db_pool.get()
    .await
    .map_err(|_|return Error::InternalError("try later #555".to_owned()))?;

    let exists:Option<String> = rd_db_pool.get(phone_number).await.map_err(|e|return Error::InternalError(e.to_string()))?;
    if exists.is_none(){
        return Err(Error::NotFoundError(String::new()))
    }

    Ok(())
}

pub async fn create_token(db_pool:&sqlx::Pool<Postgres>,token_data:&Token)->Result<(),Error>{
    sqlx::query("INSERT INTO tokens (access_token,refresh_token,user_id,session_id,status,ip,agent,created_at,expire_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)")
    .bind(token_data.access_token.clone())
    .bind(token_data.refresh_token.clone())
    .bind(token_data.user_id)
    .bind(token_data.session_id)
    .bind(token_data.status.to_string())
    .bind(token_data.ip.clone())
    .bind(token_data.agent.clone())
    .bind(token_data.created_at)
    .bind(token_data.expire_at)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    return Ok(())
}