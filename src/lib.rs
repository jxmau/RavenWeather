
pub mod config;
pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;


use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Air, NewAir, Weather, NewWeather};


pub fn establish_connection() -> PgConnection {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL must be set.");

    PgConnection::establish(&database_url)
        .expect(&format!("Eroor connecting to {}", database_url))
}
    
pub fn save_air_pollution<'a>(conn: &PgConnection, dt : &'a i64, aqi : &'a i16, co : &'a f32, no : &'a f32, no2 : &'a f32, o3 : &'a f32, so2 : &'a f32, pm2_5 : &'a f32, pm10 : &'a f32, nh3 : &'a f32) -> Air {
    use schema::air;

    let new_air = NewAir {dt : dt, aqi : aqi, co : co, no : no, no2: no2, o3 : o3, so2 : so2, pm2_5:pm2_5, pm10:pm10, nh3: nh3};

    diesel::insert_into(air::table)
        .values(&new_air)
        .get_result(conn)
        .expect("Error")
} 

pub fn save_weather<'a>(conn : &PgConnection, dt : &'a i64, wind_speed: &'a f32, wind_direction :&'a i16, temp: &'a f32, feels_like: &'a f32, temp_min: &'a f32, temp_max: &'a f32, pressure: &'a i16, humidity: &'a i16, weather_id: &'a i16) -> Weather{
    use schema::weather;

    let new_weather = NewWeather {dt: dt, wind_speed: wind_speed, wind_direction: wind_direction, temp: temp, feels_like: feels_like, temp_min:temp_min, temp_max: temp_max, pressure: pressure, humidity: humidity, weather_id: weather_id};

    diesel::insert_into(weather::table)
        .values(&new_weather)
        .get_result(conn)
        .expect("Error")
}


pub trait Trunk{
    fn trunk(&mut self) -> Self;
}

impl Trunk for f32{
    fn trunk(&mut self) -> Self{
        (*self * 100.0).round() 
    }
}