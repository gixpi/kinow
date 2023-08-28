use sqlx::Postgres;

use crate::app::types::error::Error;
use crate::rback_proto::{Empty, VerifyUserPermissionRequest};
  
pub async fn verify_user_permission(db_pool:&sqlx::Pool<Postgres>,data:VerifyUserPermissionRequest)->Result<Empty,Error>{
    let row = sqlx::query("
    SELECT user_id FROM user_roles
	INNER JOIN roles
	ON user_roles.role_id=roles.role_id
	INNER JOIN permissions
	ON roles.permission_id = permissions.permission_id
	WHERE
	user_roles.user_id=$1 AND
	permissions.permission_id = $2
    ")
    .bind(data.user_id)
    .bind(data.permission_id)
    .fetch_optional(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;

    if row.is_none(){
        return Err(Error::PermissionDeniedError("no permission #701".to_owned()))
    }
    return Ok(Empty {})
}
