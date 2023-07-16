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