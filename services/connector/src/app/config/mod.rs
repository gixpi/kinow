use clap::Parser;


#[derive(Parser, Debug)]
#[command(author = "gixpi </>", version = "0.0.1", about = "connector server",)]
pub struct ParseConfig{
    /// server address 
    /// ex.(127.0.0.1:8080)
    #[arg(required = true,long, default_value = "127.0.0.1:8083")]
    pub listen_address:String,
    
}