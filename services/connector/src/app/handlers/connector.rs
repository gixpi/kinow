use tonic::{Request, Response, Status};

use crate::app::services;
use crate::connector_proto::connector_service_server::ConnectorService;
use crate::connector_proto::{GetAccessPointRequest,GetAccessPointResponse, SetAccessPointRequest, Empty};


pub struct ConnectorHandler{
    pub connector_service:services::connector::ConnectorService
}

impl ConnectorHandler{
    pub fn new(connector_service:services::connector::ConnectorService)->Self{
        Self{
            connector_service
        }
    }
}


#[tonic::async_trait]
impl ConnectorService for ConnectorHandler{
    async fn get_access_point(&self,request:Request<GetAccessPointRequest>)->Result<Response<GetAccessPointResponse>,Status>{
        let res = self.connector_service.get_access_point(request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
    async fn set_access_point(&self,request:Request<SetAccessPointRequest>)->Result<Response<Empty>,Status>{
        let res = self.connector_service.set_access_point(request.into_inner()).await.map_err(|e| return e.to_status())?;
        Ok(Response::new(res))
    }
}