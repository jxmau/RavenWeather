use std::env;
use dotenv::dotenv;
pub struct Config {
    pub key : String,
    pub lat : String,
    pub lon : String,
}


// TODO Change to it fetches directly on the dot file.
impl Config {

    pub fn fetch() -> Config {


        // Chane the unwrap thingy to someting more safe.
        dotenv().ok();
        Config {
            key : env::var("KEY").unwrap(),
            lat : env::var("LATITUDE").unwrap(),
            lon : env::var("LONGITUDE").unwrap(),
        }
    }

}