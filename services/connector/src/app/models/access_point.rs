pub struct AccessPoint{
    pub ip:String,
    pub port:String,
    pub city:String,
}

#[derive(PartialEq, Eq,Hash)]
pub enum Point{
    Tcp,
    Udp,
    WebRTC,
    WebSocket,
}


impl From<i32> for Point{
    fn from(value: i32) -> Self {
        match value{
            0=>Self::Tcp,
            1=>Self::Udp,
            2=>Self::WebRTC,
            3=>Self::WebSocket,
            _=>Self::Tcp
        }
    }
}