


use diesel::PgConnection;
use raven_weather::{Trunk, save_weather};

use crate::config::Config;
use std::{thread, time};
use crate::request::current_weather::CurrentWeather;
use std::sync::Arc;
pub struct CurrentWeatherActualizer{
    pub config : Arc<Config>,
    pub conn: PgConnection,

}

impl CurrentWeatherActualizer {


    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

        let url : String = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", &self.config.lat, &self.config.lon, &self.config.key);

        loop {

            let response = reqwest::blocking::get(&url)?
                    .text()?;


                
            let mut weather : CurrentWeather = match serde_json::from_str(&response) {
                Ok(s) => s,
                Err(_) => CurrentWeather::new(),
            };

            weather.main.convert();

            save_weather(&self.conn, &weather.dt, &weather.wind.speed.trunk(), &weather.wind.deg, &weather.main.temp.trunk(), &weather.main.feels_like.trunk(), &weather.main.temp_min.trunk(), &weather.main.temp_max.trunk(), &weather.main.pressure, &weather.main.humidity, &weather.weather[0].id);
            
            let pause = time::Duration::from_secs(60 * 20);
            thread::sleep(pause);
        }
        
        Ok(())
    
    }


    
}
