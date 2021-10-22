use std::env;
use dotenv::dotenv;


#[derive(Clone)]
pub struct Config {
    pub key : String,
    pub lat : String,
    pub lon : String,
}


impl Config {

    pub fn fetch() -> Config {
        // Change the unwrap thingy to someting more safe.
        dotenv().ok();
        Config {
            key : env::var("KEY").unwrap(),
            lat : env::var("LATITUDE").unwrap(),
            lon : env::var("LONGITUDE").unwrap(),
        }
    }
}
