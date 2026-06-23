use reqwest::Error;
use crate::weather::{parse_latitude, parse_longitude};


//Using open weathers GEOCODING API calls instead of coordinates. https://openweathermap.org/api/geocoding-api

//City only 
//https://api.openweathermap.org/data/2.5/weather?q={city name}&appid={API key}
//Imperial format &units=imperial 
// pub async fn fetch_weather(city: &str, api_key: &str) -> Result<Value, Error> {
//     let url = format!(
//         //https://openweathermap.org/current#name
//         "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
//         city, api_key
//     );

//     let response = reqwest::get(&url).await?;
//     let json: Value = response.json().await?;
//     return Ok(json);
// }

// pub async fn fetch_weather_raw(city: &str, api_key: &str) -> Result<String, Error> {
//     let url = format!(
//         "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=imperial",
//         city, api_key
//     );

//     let response = reqwest::get(&url).await?;
//     let body = response.text().await?;
//     return Ok(body);
// }


//City/Country
//https://api.openweathermap.org/data/2.5/weather?q={city name},{country code}&appid={API key}
pub async fn fetch_weather_cc_raw(city: &str, country: &str, api_key: &str,) -> Result<String, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=imperial",
        city, country, api_key
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    return Ok(body);
}

//City/State/Country(must be United States)
pub async fn fetch_weather_csc_raw(city: &str, state: &str, country: &str, api_key: &str,) -> Result<String, Error> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={},{},{}&limit=1&appid={}&units=imperial",
        city, state, country, api_key
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    let latitude = parse_latitude(&body);
    let longitude = parse_longitude(&body);


    return Ok(fetch_weather_ll_raw(latitude, longitude, api_key).await.expect("error"));
}

//latitude longitude
//https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={API key}
async fn fetch_weather_ll_raw(lat: f64, long: f64, api_key: &str) -> Result<String, Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=imperial",
        lat, long, api_key
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    return Ok(body);
}
