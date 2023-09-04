use tonic::{Request, Response, Status};

use crate::connector_proto::connector_service_server::ConnectorService;
use crate::connector_proto::{GetAccessPointRequest,GetAccessPointResponse};
pub struct ConnectorHandler;


#[tonic::async_trait]
impl ConnectorService for ConnectorHandler{
    async fn get_access_point(&self,request:Request<GetAccessPointRequest>)->Result<Response<GetAccessPointResponse>,Status>{
        todo!("Get Access Point")
    }
}