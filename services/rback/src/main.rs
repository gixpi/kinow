use std::sync::Arc;

use clap::Parser;
use rback::app::config::ParseConfig;
use rback::app::database;
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
    // init services

    // authentication service

    println!("[INFO] Running Server On {}",parsed.listen_address);
    // Server::builder()
    // .serve(parsed.listen_address.parse().expect("could not parse the listener address"))
    // .await
    // .unwrap()
}
