mod config;
mod request;



use axum::Json;

use axum::{Router, routing::get};
use config::Config;
use diesel::{QueryDsl, RunQueryDsl};

use http::header::{self, HeaderMap, HeaderValue};
use raven_weather::establish_connection;
use raven_weather::models::{Air, Weather};

use request::air_pollution_actualizer::AirPollutionActualizer;
use request::current_weather_actualizer::CurrentWeatherActualizer;

use serde_json::{Value, json};

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


    let air = Router::new()
        .route("/air", get(|| async {(cors_headers(), get_last_air_pollution().to_json())})
            .options(|| async { cors_headers() }))
        .route("/weather", get(|| async {(cors_headers(), get_current_weather().to_json())})
            .options(|| async { cors_headers() }))
        .route("/current", get( || async {(cors_headers(), get_current() )}) 
            .options(|| async {cors_headers() }));
        
        


    axum::Server::bind(&"0.0.0.0:8666".parse().unwrap())
        .serve(air.into_make_service())
        .await
        .unwrap();


}

fn cors_headers() -> HeaderMap {

    // Change this logic to gain a &'static str
    let origin = match std::env::var("ACCESS_CONTROL_ALLOW_ORIGIN") {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    let origin = Box::new(origin);
    let origin = Box::leak(origin);

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static(origin),);
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET"),);
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("*"),);
    headers
}


// #[allow(dead_code)]
// fn security_check() -> impl Filter<Extract = (), Error = Rejection> + Copy {
//     let key = std::env::var("SECURITY_KEY").expect("SECURITY_KEY is required.");
//     let key = Box::new(key);
//     let key = Box::leak(key);
//     warp::header::exact("security-key", key)
// }



fn get_last_air_pollution() -> Air {
    use raven_weather::schema::air::dsl::*;
    use diesel::expression_methods::ExpressionMethods;

    let conn = establish_connection();

    let query_result = air.order_by(dt.desc())
        .limit(1)
        .load::<Air>(&conn)
        .expect("Error loading posts");
    
    let mut r = query_result[0];
    r.self_truncate();
    r
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

    let mut r = query_result[0];
    r.self_truncate();
    r
}


 fn get_current() -> Json<Value> {
     let air : Air = get_last_air_pollution();
     let weather : Weather = get_current_weather();
     Json(json!({
         "weather" : {
            "dt": weather.dt,
            "wind_speed" : weather.wind_speed,
            "wind_direction" : weather.wind_direction,
            "temp" : weather.temp,
            "feels_like" : weather.feels_like,
            "temp_min" : weather.temp_min,
            "temp_max" : weather.temp_max,
            "pressure" : weather.pressure,
            "humidity" : weather.humidity,
            "weather_id" : weather.weather_id,
        },
         "air": {
            "dt" :  air.dt,
            "aqi" : air.aqi,
            "co" :  air.co ,
            "no" :  air.no,
            "no2" :  air.no2,
            "o3" :  air.o3,
            "so2" :  air.so2,
            "pm2_5" :  air.pm2_5,
            "pm10" :  air.pm10,
            "nh3" :  air.nh3,
        }
     }))
 }