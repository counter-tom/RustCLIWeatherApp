
//use std::{thread, time};

pub mod util;
pub use crate::util::utility::{greet, sec_sleep, clear_screen};

mod assets;
use crate::assets::weatherassets::weather_graphics::{render_weather_image};

//use std::io;

mod openweather;
use crate::openweather::{fetch_weather_csc_raw, fetch_weather_cc_raw};

mod weather;
use crate::weather::{Weather, to_string, get_weather_code};

mod locationinput;
use crate::locationinput::{user_country,is_united_states, user_state, user_city};

use dotenv::dotenv;
use std::env;


#[tokio::main]
async fn main() {

    dotenv().ok(); 

    let api_key = env::var("OPENWEATHER_API_KEY")
        .expect("OPENWEATHER_API_KEY must be set \n Make an open weather api account at: https://home.openweathermap.org/users/sign_in \n Then subscribe to the free plan at: https://home.openweathermap.org/subscriptions");
    
    let example = r#"{"coord":{"lon":-74.5746,"lat":39.2776},"weather":[{"id":800,"main":"Clear","description":"clear sky","icon":"01n"}],"base":"stations","main":{"temp":52.12,"feels_like":50.79,"temp_min":49.17,"temp_max":54.01,"pressure":996,"humidity":80,"sea_level":996,"grnd_level":996},"visibility":10000,"wind":{"speed":7,"deg":273,"gust":17},"clouds":{"all":5},"dt":1761899929,"sys":{"type":2,"id":2006197,"country":"US","sunrise":1761909911,"sunset":1761947908},"timezone":-14400,"id":4503351,"name":"Ocean 
City","cod":200}"#;


//declarations
    //user input declarations
    let mut country = String::new();

    let mut is_usa = false;
    let mut state = String::new();

    let mut city = String::new();
    //API output string
    let mut response = String::new();
    //weather object
    let mut weather_object = Weather::new(example);
    let mut weather_code: i32 = 0;
    
    loop {
        //User Input Section
        country = user_country();

        if is_united_states(&country) {
            is_usa = true;
        }
        else {
            is_usa = false;
        }

        if is_usa {
            state = user_state();
        }

        city = user_city();

        clear_screen();
        //End User Input

        //API calls
        if is_usa {
            response = fetch_weather_csc_raw(&city, &state, &country, &api_key)
                .await
                .expect("failed");
        } 
        else {
            response = fetch_weather_cc_raw(&city, &country, &api_key)
                .await
                .expect("failed");
        }

        println!("{}",response);
        //{{"coord":{"lon":-1,"lat":39.7},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04n"}],"base":"stations","main":{"temp":47.23,"feels_like":47.23,"temp_min":47.23,"temp_max":47.23,"pressure":1017,"humidity":73,"sea_level":1017,"grnd_level":934},"visibility":10000,"wind":{"speed":2.93,"deg":82,"gust":2.53},"clouds":{"all":100},"dt":1762891809,"sys":{"country":"ES","sunrise":1762843440,"sunset":1762879932},"timezone":3600,"id":2520505,"name":"Calles","cod":200}
        weather_object = Weather::new(&response);
        //End API calls

        //Display Weather Information
        weather_code = get_weather_code(&weather_object);
        render_weather_image(weather_code);
        if is_usa{
            println!("Current weather report for: {}, {}, {}:", city, state, country);
        }
        else {
            println!("Current weather report for: {}, {}:", city, country);
        }
        
        println!("{}",to_string(&weather_object));
        //End

        //Repeat 
    }

}

