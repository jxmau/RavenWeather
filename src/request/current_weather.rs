use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CurrentWeather {
    pub dt : i64,
    pub wind : Wind,
    pub main : Main,
    pub weather : [Weather; 1],
}

impl CurrentWeather {
    pub fn new() -> CurrentWeather {
        CurrentWeather { dt: 0, wind : Wind {speed : 0.0, deg : 0}, main : Main {temp : 0.0, feels_like : 0.0, temp_max : 0.0, temp_min : 0.0, pressure : 0, humidity : 0}, weather : [ Weather {id: 0, description : "erreur".to_string()}]}
    }

}

#[derive(Serialize, Deserialize)]
pub struct Wind {
    pub speed : f32, 
    pub deg : i16,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    pub temp : f32,
    pub feels_like : f32,
    pub temp_min : f32,
    pub temp_max : f32,
    pub pressure : i16,
    pub humidity : i16,
}

impl Main {
    pub fn convert(&mut self) {
        self.temp -= 273.15;
        self.feels_like -= 273.15;
        self.temp_min -= 273.15;
        self.temp_max -= 273.15;

    }
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub id : i16,
    pub description : String,
}