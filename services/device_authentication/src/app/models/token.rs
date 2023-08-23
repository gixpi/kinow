use chrono::{Duration, Utc};

pub struct Token{
    pub device_id:i32,
    pub access_token:String,
    pub refresh_token:String,
    pub ip:String,
    pub device_type:String,
    pub status:Status,
    pub created_at:chrono::DateTime<Utc>,
    pub access_token_expire_at:chrono::DateTime<Utc>,
    pub refresh_token_expire_at:chrono::DateTime<Utc>,
}

impl Token{
    pub fn new(device_id:i32,ip:String,expiry:i32,device_type:String) -> Self{
        let access_token = idgen::alpha_numeric(32); 
        let refresh_token = idgen::alpha_numeric(32);
        Self{
            device_id,
            access_token,
            refresh_token,
            ip,
            device_type,
            status:Status::Live,
            created_at:chrono::Utc::now(),
            access_token_expire_at:chrono::Utc::now() + Duration::seconds(expiry as i64),
            refresh_token_expire_at:chrono::Utc::now() + Duration::days(30),
        }
    }
    pub fn validate_status(&self)->bool{
        match self.status{
            Status::Banned=>false,
            Status::Live=>true
        }
    }

    pub fn validate_expiry(&self)->bool{
        self.access_token_expire_at > chrono::Local::now()
    }
    
}

#[derive(PartialEq, Eq)]
pub enum Status{
    Banned,
    Live,
}

impl Default for Status{
    fn default() -> Self {
        Status::Live
    }
}

impl ToString for Status{
    fn to_string(&self) -> String {
        match self{
            Status::Banned=>format!("Banned"),
            Status::Live=>format!("Live")
        }   
    }
}

impl From<String> for Status{
    fn from(value: String) -> Self {
        let value = value.as_str();
        match value{
            "Banned"=>Self::Banned,
            "Live"=>Self::Live,
            _=>Self::Live
        }
    }
}

impl From<i32> for Status{
    fn from(value: i32) -> Self {
        match value{
            0=>Self::Banned,
            1=>Self::Live,
            _=>Self::Live,
        }
    }
}