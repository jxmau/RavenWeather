use serde_json::json;

use super::schema::air;
use super::schema::weather;

#[derive(Queryable, Copy, Clone)]
pub struct Air {
    pub id : i32, 
    pub dt : i64,
    pub aqi : i16,
    pub co : f32,
    pub no : f32,
    pub no2 : f32,
    pub o3 : f32,
    pub so2 : f32,
    pub pm2_5 : f32,
    pub pm10 : f32,
    pub nh3 : f32,
}

impl Air {
    pub fn to_json(&self) -> String {
        json!({
            "dt" :  &self.dt,
            "quality" : &self.aqi,
            "co" :  &self.co ,
            "no" :  &self.no,
            "no2" :  &self.no2,
            "o3" :  &self.o3,
            "so2" :  &self.so2,
            "pm2_5" :  &self.pm2_5,
            "pm10" :  &self.pm10,
            "nh3" :  &self.nh3,
        }).to_string()
    }
}


#[derive(Insertable)]
#[table_name="air"]
pub struct NewAir<'a> {
    pub dt : &'a i64,
    pub aqi : &'a i16,
    pub co : &'a f32,
    pub no : &'a f32,
    pub no2 : &'a f32,
    pub o3 : &'a f32,
    pub so2 : &'a f32,
    pub pm2_5 : &'a f32,
    pub pm10 : &'a f32,
    pub nh3 : &'a f32,
}

#[derive(Queryable, Copy, Clone)]
pub struct Weather {
    pub id : i32,
    pub dt : i64,
    pub wind_speed : f32,
    pub wind_direction : i16,
    pub temp : f32,
    pub feels_like : f32,
    pub temp_min : f32,
    pub temp_max : f32,
    pub pressure : i16,
    pub humidity : i16,
    pub weather_id : i16,
}

impl Weather {
    pub fn to_json(&self) -> String {
        json!({
            "dt": &self.dt,
            "wind_speed" : &self.wind_speed,
            "wind_direction" : &self.wind_direction,
            "temp" : &self.temp,
            "feels_like" : &self.feels_like,
            "temp_min" : &self.temp_min,
            "temp_max" : &self.temp_max,
            "pressure" : &self.pressure,
            "humidity" : &self.humidity,
            "weather_id" : &self.weather_id,
        }).to_string()
    }
}

#[derive(Insertable)]
#[table_name="weather"]
pub struct NewWeather<'a> {
    pub dt : &'a i64,
    pub wind_speed : &'a f32,
    pub wind_direction : &'a i16,
    pub temp : &'a f32,
    pub feels_like : &'a f32,
    pub temp_min : &'a f32,
    pub temp_max : &'a f32,
    pub pressure : &'a i16,
    pub humidity : &'a i16,
    pub weather_id : &'a i16,
}