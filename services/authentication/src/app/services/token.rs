use sqlx::{Postgres, Row};

use crate::app::models::token::Token;
use crate::token_proto::{VerificationRequest,VerificationResponse,RenewTokenRequest,TokenInfo};
use crate::app::types::error::Error;

use super::common::{self, delete_token_by_access_token_and_refresh_token};

pub async fn verify_token(db_pool:&sqlx::Pool<Postgres>,data:VerificationRequest)->Result<VerificationResponse,Error>{
    let token = common::get_token_by_access_token_and_agent(db_pool,&data.access_token, &data.agent).await?;
    if !token.validate_expiry() || !token.validate_status(){
        return Err(Error::PermissionDeniedError("Token is invalid or expired #777".to_owned()))
    }
    // we can even validate IP OR AGENT
    return Ok(VerificationResponse { user_id: token.user_id, session_id:(token.session_id as i32)})
}

pub async fn renew_token(db_pool:&sqlx::Pool<Postgres>,data:RenewTokenRequest,expiry:i32)->Result<TokenInfo,Error>{
   let row = sqlx::query("SELECT user_id FROM tokens WHERE access_token = $1 AND refresh_token = $2 AND AGENT = $3")
    .bind(data.access_token.clone())
    .bind(data.refresh_token.clone())
    .bind(data.agent.clone())
    .fetch_optional(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    if row.is_none(){
        return Err(Error::NotFoundError("token not found #404".to_owned()))
    }
    delete_token_by_access_token_and_refresh_token(db_pool, &data.access_token, &data.refresh_token).await?;
    let row = row.unwrap();
    let user_id = row.get::<i32,_>("user_id");
    let token = Token::new(user_id, data.agent, data.ip, expiry);
    common::create_token(db_pool, &token).await?;
    return Ok(TokenInfo { access_token: token.access_token, refresh_token: token.refresh_token, expiry: expiry })
}