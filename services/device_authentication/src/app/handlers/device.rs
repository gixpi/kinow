use std::sync::Arc;

use sqlx::Postgres;
use tonic::{Request, Response, Status};
use crate::app::services;
use crate::device_proto::device_service_server::DeviceService;
use crate::device_proto::{Pagination, Devices, Device, ChangeDeviceStatusRequest, Empty, DeleteDeviceRequest, GetUserDevicesRequest};



pub struct DeviceHandler{
    pub postgres_db:Arc<sqlx::Pool<Postgres>>,
}

impl DeviceHandler{
    pub fn new(postgres_db:Arc<sqlx::Pool<Postgres>>)->Self{
        Self{
            postgres_db
        }
    }
}

#[tonic::async_trait]
impl DeviceService for DeviceHandler{
    async fn get_devices(&self,request:Request<Pagination>)->Result<Response<Devices>,Status>{
        let res = services::device::get_devices(&self.postgres_db, &request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
    async fn change_device_status(&self,request:Request<ChangeDeviceStatusRequest>)->Result<Response<Empty>,Status>{
        let res = services::device::change_device_status(&self.postgres_db, &request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
    async fn delete_device(&self,request:Request<DeleteDeviceRequest>)->Result<Response<Empty>,Status>{
        let res = services::device::delete_device(&self.postgres_db, &request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
    async fn get_user_devices(&self,request:Request<GetUserDevicesRequest>)->Result<Response<Devices>,Status>{
        let res = services::device::get_user_devices(&self.postgres_db, &request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
}