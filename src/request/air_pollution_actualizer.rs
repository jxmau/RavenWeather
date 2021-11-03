use crate::config::Config;
use std::{thread, time};
use crate::request::air_pollution::AirPollution;
use diesel::pg::PgConnection;
use raven_weather::{save_air_pollution, Trunk};
use std::sync::Arc;
pub struct AirPollutionActualizer {
    pub config : Arc<Config>,
    pub conn : PgConnection,
}

impl AirPollutionActualizer {


    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {

            let url : String = format!("http://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}", &self.config.lat, &self.config.lon, &self.config.key );
            loop {

                let response : String = reqwest::blocking::get(&url)?
                    .text()?;
                    

                let mut air_received : AirPollution = match serde_json::from_str(&response) {
                    Ok(s) => s,
                    Err(_) => AirPollution::new(),
                };

                save_air_pollution(&self.conn, &air_received.list[0].dt , &air_received.list[0].main.aqi, &air_received.list[0].components.co.trunk(), &air_received.list[0].components.no.trunk(), &air_received.list[0].components.no2.trunk(), &air_received.list[0].components.o3.trunk(), &air_received.list[0].components.so2.trunk(), &air_received.list[0].components.pm2_5.trunk(), &air_received.list[0].components.pm10.trunk(), &air_received.list[0].components.nh3.trunk());
                
                let pause = time::Duration::from_secs(60 * 15);
                thread::sleep(pause);
            }
            Ok(())
    }

}