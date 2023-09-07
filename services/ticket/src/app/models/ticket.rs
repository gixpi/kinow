use chrono::{Utc, Duration};

use super::point::Point;
pub struct Ticket{
    pub ticket_id:i32,
    pub user_id:i32,
    pub point:Point,
    pub status:Status,
    pub ip:String,
    pub agent:String,
    pub expire_at:chrono::DateTime<Utc>,
}

impl Ticket{
    pub fn new(user_id:i32,point:Point,ip:String,agent:String)->Self{
        let ticket_id = idgen::numeric_code_i32(5697, 2147483647);
        Self{
            ticket_id,
            user_id,
            point,
            status:Status::OnGoing,
            ip,
            agent,
            expire_at:chrono::Utc::now() + Duration::days(1),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Status{
    OnGoing,
    Deleted,
    Banned,
}

impl ToString for Status{
    fn to_string(&self) -> String {
        match self{
            Status::OnGoing=>"OnGoing".to_owned(),
            Status::Deleted=>"Deleted".to_owned(),
            Status::Banned=>"Banned".to_owned(),
        }    
    }
}

impl Status{
    pub fn validate(&self)->bool{
        *self == Status::OnGoing
    }
}

impl From<String> for Status{
    fn from(value: String) -> Self {
        let value = value.as_str();
        match value{
            "OnGoing"=>Self::OnGoing,
            "Deleted"=>Self::Deleted,
            "Banned"=>Self::Banned,
            _=>Self::OnGoing
        }
    }
}
