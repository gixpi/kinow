
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

impl Point{
    pub fn to_i32(&self)->i32{
        match self{
            Point::Tcp=>0,
            Point::Udp=>1,
            Point::WebRTC=>2,
            Point::WebSocket=>3,
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

impl From<String> for Point{
    fn from(value: String) -> Self {
        let value = value.as_str();
        match value{
            "Tcp"=>Self::Tcp,
            "Udp"=>Self::Udp,
            "WebRTC"=>Self::WebRTC,
            "WebSocket"=>Self::WebSocket,
            _=>Self::WebSocket
        }
    }
}