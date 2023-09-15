use clap::Parser;


#[derive(Parser, Debug)]
#[command(author = "gixpi </>", version = "0.0.1", about = "rts server",)]
pub struct ParseConfig{
    /// server address 
    /// ex.(127.0.0.1:8080)
    #[arg(required = true,long, default_value = "127.0.0.1:8084")]
    pub listen_address:String,
    
    ///  [INFLUX DB. TLS]
    ///   ex.(false)
    #[arg(required = true,long)]
    pub db_tls:bool,

    ///  [INFLUX DB. HOST]
    ///   ex.(localhost OR 127.0.0.1)
    #[arg(required = true,long)]
    pub db_host:String,

    ///  [INFLUX DB. PORT]
    ///  ex.(8086)
    #[arg(required = true,long,)]
    pub db_port:usize,

    ///  [INFLUX DB. USERNAME]
    ///  ex.(INFLUX)
    #[arg(required = true,long,)]
    pub db_username:String,

    ///  [INFLUX DB. PASSWORD]
    ///  ex.(INFLUX)
    #[arg(required = true,long,)]
    pub db_password:String,

    ///  [INFLUX DB. TOKEN]
    ///  ex.(T)
    #[arg(required = true,long,)]
    pub db_token:String,

    ///  [INFLUX DB. BUCKET]
    ///  ex.(BUCKET)
    #[arg(required = true,long,)]
    pub db_bucket:String,
}