use crate::app::{models, pkg};
use crate::app::types::error::Error;
use crate::authentication_proto::{SignupRequest,OptionalResponse};

use std::sync::Arc;
use bb8_redis::{RedisConnectionManager,redis::AsyncCommands};
use sqlx::Postgres;
use super::common;

pub async fn signup(pg_db_pool:&sqlx::Pool<Postgres>, rd_db_pool:&bb8::Pool<RedisConnectionManager>, data:SignupRequest)->Result<OptionalResponse,Error>{
    let rgx = regex::Regex::new(r#"^(\+98|0)?9\d{9}$"#).map_err(|e|return Error::InternalError(e.to_string()))?;
    if !rgx.is_match(&data.phone){
        return Err(Error::ServiceError("the format of phone number is incorrect".to_owned()))
    };

    let pn_result_rd = common::phone_number_exists_rd(rd_db_pool, &data.phone).await;
    match pn_result_rd{
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

    let pn_result_pg = common::user_exists_by_phone_pg(pg_db_pool, &phone_number.clone()).await;
    match pn_result_pg{
        Ok(_)=>{},
        Err(e)=>return Err(e)
    }
    let value = serde_json::to_string(&models::user::User::new(&phone_number.clone()))
    .map_err(|_|return Error::InternalError("try later #711".to_owned()));

    let mut rd_db_pool =  rd_db_pool.get()
    .await
    .map_err(|_|return Error::InternalError("try later #555".to_owned()))?;

    let code = idgen::numeric_code_i16(1245, 9864);

    pkg::SMS::new_verification_message(code,String::from("_"), vec![phone_number.as_ref().to_owned()])
    .send_sms().await?;

    let _:() = rd_db_pool.set_ex::<String,String,()>(phone_number.as_ref().to_owned(), value.unwrap(), 120)
    .await
    .map_err(|_|return Error::InternalError("try later #712".to_owned()))?;

    let _:() = rd_db_pool.set_ex::<i16,String,()>(code, phone_number.as_ref().to_owned(), 120)
    .await
    .map_err(|_|return Error::InternalError("try later #712".to_owned()))?;
    
    // let user = User::new(&phone_number);
    // std::mem::drop(phone_number);
    // sqlx::query("INSERT INTO users (user_id,phone_number,status,created_at) VALUES ($1,$2,$3,$4)")
    // .bind(user.user_id)
    // .bind(user.phone_number)
    // .bind(user.status.to_string())
    // .bind(user.created_at)
    // .execute(pg_db_pool)
    // .await
    // .map_err(|e|return Error::InternalError(e.to_string()))?;
    
    // let token = Token::new(user.user_id, data.agent, data.ip);
    // return Ok(TokenInfo { access_token: token.access_token, refresh_token: token.refresh_token, expiry: 300 })
    // let token = Token::new(user.user_id, data.agent, data.ip);
    // Ok(TokenInfo { access_token: token.access_token, refresh_token: token.refresh_token, expiry: 300 })
    Ok(OptionalResponse {  msg: None,code:Some(code.to_string()) })
}