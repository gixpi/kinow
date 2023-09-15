use influxdb::Client;
use super::types::error::Error;
pub async fn influxdb_connection(tls:bool,host:String,port:usize,database:String,token:String)->Result<Client,Error>{
    let url = if tls{
        format!("https://{host}:{port}/")
    }else{
        format!("http://{host}:{port}/")
    };
    let c=  Client::new(url, database)
    .with_token(token);
    c.ping().await
    .map_err(|e|return Error::DBPoolError(e.to_string())).unwrap();
    return Ok(c)
}
