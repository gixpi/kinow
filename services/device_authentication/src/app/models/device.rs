use chrono::Utc;

pub struct Device{
    pub device_id:i32,
    pub device_type:String,
    pub serial_code:String,
    pub device_status:Status,
    pub lock_code:String,
    pub user_id:i32,
    pub created_at:chrono::DateTime<Utc>,
}

impl Device{
    pub fn new(serial_code:String,device_type:String,user_id:i32)->Self{
        let device_id = idgen::numeric_code_i32(100647,2147483647);
        let lock_code = idgen::alpha_numeric(12);
        
        Self{
            device_id,
            device_type,
            serial_code,
            device_status:Status::Offline,
            lock_code,
            user_id,
            created_at:chrono::Utc::now(),
        }
    }
}
#[derive(PartialEq ,Eq)]
pub enum Status{
   Online,
    Idle,
    Offline
}

impl Default for Status{
    fn default() -> Self {
        Self::Offline
    }
}

impl ToString for Status{
    fn to_string(&self) -> String {
        match self{
            Status::Online=>"Online".to_owned(),
            Status::Idle=>"Idle".to_owned(),
            Status::Offline=>"Offline".to_owned(),
        }
    }
}

impl From<String> for Status{
    fn from(value: String) -> Self {
        let value = value.as_str(); 
        match value{
            "Online"=>Self::Online,
            "Idle"=>Self::Idle,
            "Offline"=>Self::Offline,
            _=>Self::Offline
        }
    }
}

impl From<i32> for Status{
    fn from(value: i32) -> Self {
        match value{
            0=>Self::Online,
            1=>Self::Idle,
            2=>Self::Offline,
            _=>Self::Offline
        }
    }
}