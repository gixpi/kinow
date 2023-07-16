use chrono::{Local, Timelike};

pub struct Token{
    user_id:usize,
    access_token:String,
    refresh_token:String,
    session_id:i16,
    agent:String,
    ip:String,
    status:Status,
    created_at:chrono::DateTime<Local>,
    expire_at:chrono::DateTime<Local>,
}

impl Token{
    pub fn new(user_id:usize,agent:String,ip:String) -> Self{
        let access_token = idgen::alpha_numeric(32); 
        let refresh_token = idgen::alpha_numeric(32);
        let session_id = idgen::numeric_code_i16(156,32767);
        Self{
            user_id,
            access_token,
            refresh_token,
            session_id,
            agent,
            ip,
            status:Status::Live,
            created_at:chrono::Local::now(),
            expire_at:chrono::Local::now().with_minute(5).unwrap(),
        }
    }

    pub fn validate_status(&self)->bool{
        match self.status{
            Status::Banned=>false,
            Status::Live=>true
        }
    }

    pub fn validate_expiry(&self)->bool{
        if self.expire_at > chrono::Local::now(){
            return true
        }   
        return false
    }
}

enum Status{
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