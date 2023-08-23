use std::sync::Arc;

use clap::Parser;
use device_authenticationlib::app::config::ParseConfig;
use device_authenticationlib::app::database;
use device_authenticationlib::app::handlers;
use device_authenticationlib::authentication_proto::authentication_service_server::AuthenticationServiceServer;
use device_authenticationlib::token_proto::token_service_server::TokenServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    
    println!("[INFO] Parse Input Config");
    let parsed =  ParseConfig::parse();
    println!("[INFO] Connecting To PostgresDB...");
    let pg_db_pool = database::postgres_connection(parsed.db_username, parsed.db_password, parsed.db_host, parsed.db_port,parsed.db_name)
    .await.unwrap();
    println!("[INFO] Connected To PostgresDB!");
    
    let pg_db_pool = Arc::new(pg_db_pool);
    
    println!("[INFO] Init Services");
    
    let authentication_handler = handlers::authentication::AuthenticationHandler::new(pg_db_pool.clone(),parsed.token_life_expiry.clone());
    let authentication_service = AuthenticationServiceServer::new(authentication_handler);

    let token_handler = handlers::token::TokenHandler::new(pg_db_pool.clone(),parsed.token_life_expiry.clone());
    let token_service = TokenServiceServer::new(token_handler);

    println!("[INFO] Running Server On {}",parsed.listen_address);
    Server::builder()
    .add_service(authentication_service)
    .add_service(token_service)
    .serve(parsed.listen_address.parse().expect("could not parse the listener address"))
    .await
    .unwrap()
}
