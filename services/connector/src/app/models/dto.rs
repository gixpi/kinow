use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ip2Location{
    pub status:String,
    pub city:Option<String>
}