
pub enum Point{
    Tcp,
    Udp,
    WebRTC,
    WebSocket,
}

impl From<i32> for Point{
    fn from(value: i32) -> Self {
        match value{
            0=>Point::Tcp,
            1=>Point::Udp,
            2=>Point::WebRTC,
            3=>Point::WebSocket,
            _=>Point::WebSocket,
        }
    }
}

impl ToString for Point{
    fn to_string(&self) -> String {
        match self{
            Self::Tcp=>"Tcp".to_owned(),
            Self::Udp=>"Udp".to_owned(),
            Self::WebRTC=>"WebRTC".to_owned(),
            Self::WebSocket=>"WebSocket".to_owned(),
        }   
    }
}