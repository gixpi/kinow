use std::sync::Arc;

use sqlx::{Postgres, Row};
use crate::app::models::device::Device;
use crate::app::models::token::Token;
use crate::app::services::common::device_exists_by_serial_and_user_id_pg;
use crate::authentication_proto::{TokenInfo,SignupRequest, SigninRequest, SignupResponse};
use crate::app::types::error::Error;

use super::common;

pub async fn signup(db_pool:&sqlx::Pool<Postgres>,data:SignupRequest)->Result<SignupResponse,Error>{
    let ow_device = Arc::new(data);
    device_exists_by_serial_and_user_id_pg(db_pool, &ow_device.serial_code, &ow_device.user_id)
    .await?;
    let device = Device::new(ow_device.serial_code.to_owned(), ow_device.device_type.to_owned(), ow_device.user_id); 
    sqlx::query("INSERT INTO devices (device_id,device_type,serial_code,device_status,lock_code,user_id,created_at) VALUES ($1,$2,$3,$4,$5,$6,$7)")
    .bind(device.device_id)
    .bind(device.device_type)
    .bind(device.serial_code.clone())
    .bind(device.device_status.to_string())
    .bind(device.lock_code.clone())
    .bind(device.user_id)
    .bind(device.created_at)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;

    Ok(SignupResponse { serial_code: device.serial_code, lock_code: device.lock_code })
}

pub async fn signin(db_pool:&sqlx::Pool<Postgres>,data:SigninRequest,token_life_expiry:i32)->Result<TokenInfo,Error>{
    let ow_data = Arc::new(data);
    let res = sqlx::query("SELECT device_id FROM devices WHERE serial_code = $1 AND lock_code = $2")
    .bind(ow_data.serial_code.clone())
    .bind(ow_data.lock_code.clone())
    .fetch_optional(db_pool)
    .await
    .map_err(|e| Error::InternalError(e.to_string()))?;

    if res.is_none(){
        return Err(Error::ServiceError(format!("serial code or lock code is incorrect #400")))
    }
    let res = res.unwrap();
    let device_id = res.get::<i32,_>("device_id");
    let token = Token::new(device_id, ow_data.ip.to_owned(), token_life_expiry);
    common::create_token(db_pool, &token).await?;
    Ok(TokenInfo { access_token: token.access_token, refresh_token: token.refresh_token, expiry: token_life_expiry})

}