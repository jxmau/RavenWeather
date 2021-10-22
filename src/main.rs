mod config;
mod request;



use config::Config;
use diesel::{QueryDsl, RunQueryDsl};
use raven_weather::establish_connection;
use raven_weather::models::{Air, Weather};

use request::air_pollution_actualizer::AirPollutionActualizer;
use request::current_weather_actualizer::CurrentWeatherActualizer;
use serde_json::json;
use warp::Filter;
use std::sync::Arc;



#[tokio::main]
async fn main() {

    let config : Arc<Config> = Arc::new(Config::fetch());
    let config_clone = config.clone();

    let _thread_1 = std::thread::spawn(move || {
        let air_pollution_actualizer = AirPollutionActualizer {config:  config_clone, conn : establish_connection()};
        air_pollution_actualizer.run();
    });

    let _thread_2 = std::thread::spawn(move || {
        let current_weather_actualizer = CurrentWeatherActualizer {config, conn: establish_connection()};
        current_weather_actualizer.run();
    });



    warp::serve(get_routes()).run(([127, 0, 0, 1], 8666)).await;
}


fn get_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {

    let air = warp::path!("air").map(|| get_last_air_pollution().to_json());
    let weather = warp::path!("weather").map(|| get_current_weather().to_json());
    let current_condition = warp::path!("current-condition").map(|| get_current());


    let routes = warp::get().and(air.or(weather.or(current_condition)));
    routes

}



fn get_last_air_pollution() -> Air {
    use raven_weather::schema::air::dsl::*;
    use diesel::expression_methods::ExpressionMethods;

    let conn = establish_connection();

    let query_result = air.order_by(dt.desc())
        .limit(1)
        .load::<Air>(&conn)
        .expect("Error loading posts");
    
    query_result[0]
}

fn get_current_weather() -> Weather {
    use raven_weather::schema::weather::dsl::*;
    use diesel::expression_methods::ExpressionMethods;

    let conn = establish_connection();

    // When Forecast will be implemented, 
    //a Filter must be used to get the biggest dt that is inferior of current epoch Time.
    let query_result = weather.order_by(dt.desc())
        .limit(1)
        .load::<Weather>(&conn)
        .expect("Error loading Weather post.");

    query_result[0]
}

fn get_current() -> String {
    // ISSUE -> Logically, it returns two String, and not two JSON Object.

    let air : Air = get_last_air_pollution();
    let weather : Weather = get_current_weather();
    json!({
        "current_weather": {
            "dt": weather.dt,
            "wind_speed" : &weather.wind_speed,
            "wind_direction" : &weather.wind_direction,
            "temp" : &weather.temp,
            "feels_like" : &weather.feels_like,
            "temp_min" : &weather.temp_min,
            "temp_max" : &weather.temp_max,
            "pressure" : &weather.pressure,
            "humidity" : &weather.humidity,
            "weather_id" : &weather.weather_id,
        },
        "current_air_pollution": {
            "dt" :  &air.dt,
            "quality" : &air.aqi,
            "co" :  &air.co ,
            "no" :  &air.no,
            "no2" :  &air.no2,
            "o3" :  &air.o3,
            "so2" :  &air.so2,
            "pm2_5" :  &air.pm2_5,
            "pm10" :  &air.pm10,
            "nh3" :  &air.nh3,
        },
    }).to_string()
}