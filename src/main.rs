mod config;
mod request;



use config::Config;
use diesel::{QueryDsl, RunQueryDsl};
use raven_weather::establish_connection;
use raven_weather::models::{Air, Weather};

use request::air_pollution_actualizer::AirPollutionActualizer;
use request::current_weather_actualizer::CurrentWeatherActualizer;

use serde::{Serialize, Deserialize};
use warp::reply::json;
use warp::{Filter, Rejection, Reply};
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

    let air = warp::path("air")
        .and(security_check())
        .map(|| json(&get_last_air_pollution()).into_response());

    let weather = warp::path!("weather")
        .and(security_check())
        .map(|| json(&get_current_weather()).into_response());

    let current_condition = warp::path!("current-condition")
        .and(security_check())
        .map(|| json(&get_current()).into_response());

    let routes = warp::get().and(
        air
        .or(weather)
        .or(current_condition)
    );
    routes   
}


fn security_check() -> impl Filter<Extract = (), Error = Rejection> + Copy {

    let key = std::env::var("SECURITY_KEY").expect("SECURITY_KEY is required.");
    let key = Box::new(key);
    let key = Box::leak(key);
    warp::header::exact("security-key", key)
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


#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CurrentCondition {
    current_air_pollution: Air,
    current_weather_condition: Weather,
}

fn get_current() -> CurrentCondition {

    let air : Air = get_last_air_pollution();
    let weather : Weather = get_current_weather();
    let current = CurrentCondition {current_air_pollution : air, current_weather_condition: weather};
    current
}