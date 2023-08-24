use chrono::Utc;
use sqlx::{Postgres, Row};

use crate::app::models::device;
use crate::device_proto::{Pagination, Devices, Device, ChangeDeviceStatusRequest, Empty, DeleteDeviceRequest,GetUserDevicesRequest};
use crate::app::types::error::Error;

pub async fn get_devices(db_pool:&sqlx::Pool<Postgres>,data:&Pagination)->Result<Devices,Error>{
    let rows = sqlx::query("SELECT device_id,device_type,serial_code,lock_code,device_status,created_at FROM devices")
    .fetch_all(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    
    let mut devices = Vec::<Device>::new();
    for row in rows{
        let device = Device{
            device_id:row.get::<i32,_>("device_id"),
            device_type:row.get::<String,_>("device_type"),
            device_status:row.get::<String,_>("device_status"),
            serial_code:row.get::<String,_>("serial_code"),
            lock_code:row.get::<String,_>("lock_code"),
            created_at:row.get::<chrono::DateTime<Utc>,_>("created_at").to_string()
        };
        devices.push(device);
    }
    if data.get_total{
        let count = sqlx::query("SELECT Count(device_id  ) AS count FROM devices")
        .fetch_one(db_pool)
        .await
        .map_err(|e|return Error::InternalError(e.to_string()))?;
        return Ok(Devices { devices:devices, total_count: Some(count.get::<i64,_>("count")) })
    }
    Ok(Devices { devices:devices,total_count: None })
}

pub async fn change_device_status(db_pool:&sqlx::Pool<Postgres>,data:&ChangeDeviceStatusRequest)->Result<Empty,Error>{
    let row: Option<sqlx::postgres::PgRow> = sqlx::query("SELECT device_status FROM devices WHERE device_id = $1")
    .bind(data.device_id.clone())
    .fetch_optional(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    if row.is_none(){
        return Err(Error::NotFoundError("device not found #404".to_owned()))
    }

    let row = row.unwrap();
    let device_status = row.get::<String,_>("device_status");

    let device_status = device::Status::from(device_status);
    let data_device_status = device::Status::from(data.device_status); 

    if device_status == data_device_status{
        return Err(Error::ServiceError(format!("the device status alread is {} #400",data_device_status.to_string())))
    }

    sqlx::query("UPDATE devices SET device_status = $1 WHERE device_id = $2")
    .bind(data_device_status.to_string())
    .bind(data.device_id.clone())
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;

    Ok(Empty{})
}

pub async fn delete_device(db_pool:&sqlx::Pool<Postgres>,data:&DeleteDeviceRequest)->Result<Empty,Error>{
    sqlx::query("DELETE FROM devices WHERE device_id = $1")
    .bind(data.device_id)
    .execute(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    Ok(Empty{})
}

pub async fn get_user_devices(db_pool:&sqlx::Pool<Postgres>,data:&GetUserDevicesRequest)->Result<Devices,Error>{
    let rows = sqlx::query("SELECT device_id,device_type,serial_code,lock_code,device_status,created_at FROM devices WHERE user_id = $1")
    .bind(data.user_id)
    .fetch_all(db_pool)
    .await
    .map_err(|e|return Error::InternalError(e.to_string()))?;
    
    let mut devices = Vec::<Device>::new();
    for row in rows{
        let device = Device{
            device_id:row.get::<i32,_>("device_id"),
            device_type:row.get::<String,_>("device_type"),
            device_status:row.get::<String,_>("device_status"),
            serial_code:row.get::<String,_>("serial_code"),
            lock_code:row.get::<String,_>("lock_code"),
            created_at:row.get::<chrono::DateTime<Utc>,_>("created_at").to_string()
        };
        devices.push(device);
    }
    let pagination = data.pagination.to_owned();
    if pagination.is_some(){
        if pagination.unwrap().get_total{
            let count = sqlx::query("SELECT Count(device_id) AS count FROM devices WHERE user_id = $1")
            .bind(data.user_id)
            .fetch_one(db_pool)
            .await
            .map_err(|e|return Error::InternalError(e.to_string()))?;
            return Ok(Devices { devices:devices, total_count: Some(count.get::<i64,_>("count")) })
        }
    }
    Ok(Devices { devices:devices,total_count: None })
}