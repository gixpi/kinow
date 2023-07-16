use crate::app::models;
use crate::app::types::error::Error;
use crate::authentication_proto::{SignupRequest,TokenInfo};

use std::sync::Arc;
use bb8_redis::{RedisConnectionManager,redis::AsyncCommands};
use sqlx::Postgres;
use super::common;

pub async fn signup(pg_db_pool:&sqlx::Pool<Postgres>, rd_db_pool:&bb8::Pool<RedisConnectionManager>, data:SignupRequest)->Result<TokenInfo,Error>{
    let rgx = regex::Regex::new(r#"^(\+98|0)?9\d{9}$"#).map_err(|e|return Error::InternalError(e.to_string()))?;
    if !rgx.is_match(&data.phone){
        return Err(Error::ServiceError("the format of phone number is incorrect".to_owned()))
    };

    let pn_result = common::phone_number_exists_rd(rd_db_pool, &data.phone).await;
    match pn_result{
        Ok(_)=>return Err(Error::ServiceError("you've already made a proccess line. try later".to_owned())),
        Err(e)=>{
            match e{
                Error::NotFoundError(_) => {},
                Error::InternalError(e) => return Err(Error::InternalError(e)),
                _=>return Err(Error::InternalError("rgx #111".to_owned())),
            }
        }
    }
    let phone_number = Arc::new(data.phone);

    common::user_exists_by_phone_pg(pg_db_pool, &phone_number.clone()).await?;
    
    let value = serde_json::to_string(&models::user::User::new(&phone_number.clone()))
    .map_err(|_|return Error::InternalError("try later #711".to_owned()));

    let mut rd_db_pool =  rd_db_pool.get()
    .await
    .map_err(|_|return Error::InternalError("try later #555".to_owned()))?;

    let _:() = rd_db_pool.set_ex::<String,String,()>(phone_number.as_ref().to_owned(), value.unwrap(), 10)
    .await
    .map_err(|_|return Error::InternalError("try later #712".to_owned()))?;


    drop(phone_number);
    // sqlx::query("INSERT INTO users (user_id,phone_number,status,created_at) VALUES ($1,$2,$3,$4)")
    // .bind(user.user_id)
    // .bind(user.phone)
    // .bind(user.status.to_string())
    // .bind(user.created_at)
    // .execute(pg_db_pool)
    // .await
    // .map_err(|e|return Error::InternalError(e.to_string()))?;
    Ok(TokenInfo { access_token: String::from("access_token"), refresh_token: String::from("refresh_token"), expiry: 300 })
}