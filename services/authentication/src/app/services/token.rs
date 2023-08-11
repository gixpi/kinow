use chrono::Utc;
use sqlx::{Postgres, Row};

use crate::app::models::token;
use crate::token_proto::{VerificationRequest,VerificationResponse,RenewTokenRequest,TokenInfo, ChangeTokenStatusRequest, Empty, Pagination, Tokens,Token};
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
    let token = token::Token::new(user_id, data.agent, data.ip, expiry);
    common::create_token(db_pool, &token).await?;
    return Ok(TokenInfo { access_token: token.access_token, refresh_token: token.refresh_token, expiry: expiry })
}

pub async fn change_token_status(db_pool:&sqlx::Pool<Postgres>,data:ChangeTokenStatusRequest)->Result<Empty,Error>{
    let row = sqlx::query("SELECT token_status FROM tokens WHERE access_token = $1")
    .bind(data.access_token.clone())
    .fetch_optional(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    if row.is_none(){
        return Err(Error::NotFoundError("token not found #404".to_owned()))
    }

    let row = row.unwrap();
    let token_status = row.get::<String,_>("token_status");

    let token_status = token::Status::from(token_status);
    let data_token_status = token::Status::from(data.token_status); 

    if token_status == data_token_status{
        return Err(Error::ServiceError(format!("the token status alread is {} #400",data_token_status.to_string())))
    }

    sqlx::query("UPDATE tokens SET token_status = $1 WHERE access_token = $2")
    .bind(data_token_status.to_string())
    .bind(data.access_token.clone())
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;

    return Ok(Empty{})
}

pub async fn get_tokens(db_pool:&sqlx::Pool<Postgres>,data:Pagination)->Result<Tokens,Error>{
    let rows = sqlx::query("SELECT access_token,refresh_token,user_id,session_id,token_status,ip,agent,created_at,access_token_expire_at,refresh_token_expire_at FROM tokens OFFSET $1 LIMIT $2")
    .bind(data.offset)
    .bind(data.limit)
    .fetch_all(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    let mut tokens = Vec::<Token>::new();
    for row in rows{
        let token = Token{
            user_id:row.get::<i32,_>("user_id"),
            access_token:row.get::<String,_>("access_token"),
            refresh_token:row.get::<String,_>("refresh_token"),
            session_id:row.get::<i16,_>("session_id") as i32,
            agent:row.get::<String,_>("agent"),
            ip:row.get::<String,_>("ip"),
            token_status:row.get::<String,_>("token_status"),
            created_at: row.get::<chrono::DateTime<Utc>,_>("created_at").to_string(),
            access_token_expire_at: row.get::<chrono::DateTime<Utc>,_>("access_token_expire_at").to_string(),
            refresh_token_expire_at: row.get::<chrono::DateTime<Utc>,_>("refresh_token_expire_at").to_string(),
        };
        tokens.push(token)
    }

    if data.get_total{
        let count = sqlx::query("SELECT Count(access_token) AS count FROM tokens")
        .fetch_one(db_pool)
        .await
        .map_err(|e|return Error::InternalError(e.to_string()))?;
        return Ok(Tokens { tokens: tokens,total_count:Some(count.get::<i64,_>("count")) })
    }
    return Ok(Tokens { tokens: tokens,total_count:None })
}