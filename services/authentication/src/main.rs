use std::sync::Arc;

use authenticationlib::app::config::ParseConfig;
use authenticationlib::app::{database, handlers};
use authenticationlib::authentication_proto::authentication_service_server::AuthenticationServiceServer;
use clap::Parser;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    
    println!("[INFO] Parse Input Config");
    let parsed =  ParseConfig::parse();
    println!("[INFO] Connecting To PostgresDB...");
    let pg_db_pool = database::postgres_connection(parsed.db_username, parsed.db_password, parsed.db_host, parsed.db_port,parsed.db_name)
    .await.unwrap();
    println!("[INFO] Connected To PostgresDB!");
    
    println!("[INFO] Connecting To Redis...");
    let rd_db_pool = database::redis_connection(parsed.redis_host)
    .await.unwrap();
    println!("[INFO] Connected To Redis!");

    let rd_db_pool = Arc::new(rd_db_pool);
    let pg_db_pool = Arc::new(pg_db_pool);
    
    println!("[INFO] Init Services");
    // init services

    // authentication service
    let authentication_handler = handlers::authentication::AuthenticationHandler::new(pg_db_pool.clone(),rd_db_pool.clone(),parsed.token_life_expiry);
    let authentication_service = AuthenticationServiceServer::new(authentication_handler);

    println!("[INFO] Running Server On {}",parsed.listen_address);
    Server::builder()
    .add_service(authentication_service)
    .serve(parsed.listen_address.parse().expect("could not parse the listener address"))
    .await
    .unwrap()
}
