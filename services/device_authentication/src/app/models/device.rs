use chrono::Utc;

pub struct Device{
    pub device_id:i32,
    pub device_type:String,
    pub serial_code:String,
    pub device_status:DeviceStatus,
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
            device_status:DeviceStatus::Offline,
            lock_code,
            user_id,
            created_at:chrono::Utc::now(),
        }
    }
}
pub enum DeviceStatus{
   Online,
    Idle,
    Offline
}

impl Default for DeviceStatus{
    fn default() -> Self {
        Self::Offline
    }
}

impl ToString for DeviceStatus{
    fn to_string(&self) -> String {
        match self{
            DeviceStatus::Online=>"Online".to_owned(),
            DeviceStatus::Idle=>"Idle".to_owned(),
            DeviceStatus::Offline=>"Offline".to_owned(),
        }
    }
}

impl From<String> for DeviceStatus{
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