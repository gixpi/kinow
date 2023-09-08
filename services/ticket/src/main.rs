use std::sync::Arc;

use ticketlib::ticket_proto::ticket_service_server::TicketServiceServer;
use ticketlib::app::config::ParseConfig;
use ticketlib::app::{ handlers, database};
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
    // init services

    let pg_db_pool = Arc::new(pg_db_pool);


    let ticket_handler = handlers::ticket::TicketHandler::new(pg_db_pool);
    let ticket_service = TicketServiceServer::new(ticket_handler);



    println!("[INFO] Running Server On {}",parsed.listen_address);
    Server::builder()
    .add_service(ticket_service)
    .serve(parsed.listen_address.parse().expect("could not parse the listener address"))
    .await
    .unwrap()
}
