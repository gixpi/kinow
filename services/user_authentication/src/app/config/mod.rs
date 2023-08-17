use clap::Parser;


#[derive(Parser, Debug)]
#[command(author = "gixpi </>", version = "0.0.1", about = "user authentication server",)]
pub struct ParseConfig{
    /// server address 
    /// ex.(127.0.0.1:8080)
    #[arg(required = true,long, default_value = "127.0.0.1:8080")]
    pub listen_address:String,
    
    ///  [POSTGRES DB. HOST]
    ///   ex.(localhost OR 127.0.0.1)
    #[arg(required = true,long)]
    pub db_host:String,

    ///  [POSTGRES DB. PORT]
    ///  ex.(5432)
    #[arg(required = true,long,)]
    pub db_port:usize,

    ///  [POSTGRES DB. USERNAME]
    ///  ex.(postgres)
    #[arg(required = true,long,)]
    pub db_username:String,

    ///  [POSTGRES DB. PASSWORD]
    ///  ex.(postgres)
    #[arg(required = true,long,)]
    pub db_password:String,

    ///  [POSTGRES DB. NAME]
    ///  ex.(postgres)
    #[arg(required = true,long,)]
    pub db_name:String,

    ///  [REDIS DB. HOST]
    ///  ex.(127.0.0.1)
    #[arg(required = true,long,)]
    pub redis_host:String,

    ///  [TOKEN LIFE EXPIRY IN SECONDS]
    ///  ex.(2000)
    #[arg(required = true,long,)]
    pub token_life_expiry:i32,
}