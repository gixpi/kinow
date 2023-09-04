use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::app::models::access_point::{AccessPoint, Point};
use crate::app::models::dto::Ip2Location;
use crate::connector_proto::{GetAccessPointResponse, GetAccessPointRequest, SetAccessPointRequest, Empty};
use crate::app::types::error::Error;



pub struct ConnectorService {
    lists: Arc<RwLock<HashMap<Point, Vec<AccessPoint>>>>,
}

impl ConnectorService {
    pub fn new() -> Self {
        ConnectorService {
            lists: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_access_point(&self, data: GetAccessPointRequest) -> Result<GetAccessPointResponse, Error> {
        let res = reqwest::get(format!("http://ip-api.com/json/{}?fields=city,status", data.ip))
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?
            .json::<Ip2Location>()
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;


        if res.status != "success"{
            return Err(Error::ServiceError("could not find #404".to_owned()))
        }

        let city = res.city.unwrap();
        let point = Point::from(data.point);
        let lists = self.lists.read().unwrap();
        if let Some(vap) = lists.get(&point) {
            if vap.is_empty(){
                return Err(Error::ServiceError("not server find #404".to_owned()))
            }
            for i in vap {
                if i.city == city {
                    return Ok(GetAccessPointResponse {
                        ip: i.ip.to_owned(),
                        port: i.port.to_owned(),
                    });
                }
            }
            return Ok(GetAccessPointResponse {
                ip: vap[0].ip.to_owned(),
                port: vap[0].port.to_owned(),
            });
        }
        Err(Error::NotFoundError("not found #404".to_owned()))
    }

    pub async fn set_access_point(&self, data: SetAccessPointRequest) -> Result<Empty, Error> {
        let res = reqwest::get(format!("http://ip-api.com/json/{}?fields=city,status", data.ip))
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?
            .json::<Ip2Location>()
            .await
            .map_err(|e| Error::InternalError(e.to_string()))?;

        if res.status != "success"{
            return Err(Error::ServiceError("could not find #404".to_owned()))
        }
        let city = res.city.unwrap();


        let ap = AccessPoint {
            city: city,
            ip: data.ip,
            port: data.port,
        };
        let point = Point::from(data.point);

        let mut lists = self.lists.write().unwrap();
        if let Some(vec) = lists.get_mut(&point) {
            for existing_ap in vec.iter() {
                if existing_ap.ip == ap.ip && existing_ap.port == ap.port {
                    return Err(Error::ServiceError("already added #409".to_owned()));
                }
            }
            vec.push(ap);
        } else {
            let new_vec = vec![ap];
            lists.insert(point, new_vec);
        }

        Ok(Empty {})
    }
}