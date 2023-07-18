use chrono::Local;
use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize)]
pub struct User{
    pub user_id:i32,
    pub phone_number:String,
    pub status:Status,
    pub created_at:chrono::DateTime<Local>,
}

impl User{
    pub fn new(phone_number:&String)->Self{
        let user_id = idgen::numeric_code_i32(100483647,2147483647);
        Self{
            user_id,
            phone_number:phone_number.to_owned(),
            status:Status::default(),
            created_at:chrono::Local::now(),
        }
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub enum Status{
    OnGoing(chrono::DateTime<Local>),
    Suspended(chrono::DateTime<Local>),
    Deleted(chrono::DateTime<Local>),
    PermanentBan(chrono::DateTime<Local>),
}

impl Default for Status{
    fn default() -> Self {
        Self::OnGoing(chrono::Local::now())    
    }
}

impl ToString for Status{
    fn to_string(&self) -> String {
        match self{
            Status::OnGoing(e)=>format!("OnGoing {}",e),
            Status::Suspended(e)=>format!("Suspended {}",e),
            Status::Deleted(e)=>format!("Deleted {}",e),
            Status::PermanentBan(e)=>format!("PermanentBan {}",e)
        }
    }
}

impl From<String> for Status{
    fn from(value: String) -> Self {
        let (status,time) = value.split_once(' ').unwrap();

        match status{
            "OnGoing"=>Self::OnGoing(time.parse::<chrono::DateTime<Local>>().unwrap()),
            "Suspended"=>Self::Suspended(time.parse::<chrono::DateTime<Local>>().unwrap()),
            "Deleted"=>Self::Deleted(time.parse::<chrono::DateTime<Local>>().unwrap()),
            _=>Self::OnGoing(time.parse::<chrono::DateTime<Local>>().unwrap())
        }
    }
}
