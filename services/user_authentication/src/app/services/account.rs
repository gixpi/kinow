use chrono::Utc;
use sqlx::{Postgres, Row};

use crate::account_proto::{LogoutRequest, Empty, KillSessionRequest, Sessions, GetSessionsRequest, Session};
use crate::app::types::error::Error;

pub async fn logout(db_pool:&sqlx::Pool<Postgres>,data:LogoutRequest)->Result<Empty,Error>{
    sqlx::query("DELETE FROM tokens WHERE access_token = $1 AND user_id = $2")
    .bind(data.access_token)
    .bind(data.user_id)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    Ok(Empty{})
}

pub async fn kill_session(db_pool:&sqlx::Pool<Postgres>,data:KillSessionRequest)->Result<Empty,Error>{
    sqlx::query("DELETE FROM tokens WHERE session_id = $1")
    .bind(data.session_id)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    Ok(Empty{})
}

pub async fn get_sessions(db_pool:&sqlx::Pool<Postgres>,data:GetSessionsRequest)->Result<Sessions,Error>{
    let rows = sqlx::query("SELECT session_id,agent,ip,created_at,token_status FROM tokens WHERE user_id = $1")
    .bind(data.user_id)
    .fetch_all(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;

    let mut sessions = Vec::<Session>::new();

    for row in rows{
        let session = Session{
            agent:row.get::<String,_>("agent"),
            ip:row.get::<String,_>("ip"),
            created_at:row.get::<chrono::DateTime<Utc>,_>("created_at").to_string(),
            session_id:row.get::<i16,_>("session_id") as i32,
            status:row.get::<String,_>("token_status"),
        };
        sessions.push(session)
    }
    Ok(Sessions { sessions: sessions })
}