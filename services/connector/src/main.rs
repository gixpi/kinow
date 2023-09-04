use connectorlib::connector_proto::connector_service_server::ConnectorServiceServer;
use connectorlib::app::config::ParseConfig;
use connectorlib::app::{ handlers, services};
use clap::Parser;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    
    println!("[INFO] Parse Input Config");
    let parsed =  ParseConfig::parse();

    // init services
    let connector_service = services::connector::ConnectorService::new();

    let authentication_handler = handlers::connector::ConnectorHandler::new(connector_service);
    let authentication_service = ConnectorServiceServer::new(authentication_handler);



    println!("[INFO] Running Server On {}",parsed.listen_address);
    Server::builder()
    .add_service(authentication_service)
    .serve(parsed.listen_address.parse().expect("could not parse the listener address"))
    .await
    .unwrap()
}
