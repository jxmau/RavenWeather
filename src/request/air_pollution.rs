

use serde::{Serialize, Deserialize};


#[derive( Serialize, Deserialize)]
pub struct AirPollution {
    pub list : [Element; 1],
}

impl AirPollution {

    pub fn new() -> AirPollution {
        AirPollution {list : [Element {dt: 0, main : Main { aqi : 0}, components : Components {co : 0.0, no : 0.0, no2 : 0.0, o3 : 0.0, so2 : 0.0, pm2_5: 0.0, pm10: 0.0, nh3: 0.0}}]}
    }

}


#[derive(Serialize, Deserialize)]
pub struct Element {
    pub dt : i64,
    pub main : Main, 
    pub components : Components,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    pub aqi : i16,
}

#[derive(Serialize, Deserialize)]
pub struct Components {
    pub co : f32,
    pub no : f32,
    pub no2 : f32,
    pub o3 : f32,
    pub so2 : f32,
    pub pm2_5 : f32,
    pub pm10 : f32,
    pub nh3 : f32,
}

