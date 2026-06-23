use regex::Regex;

    pub struct Weather {
        weather_code: i32,
        weather_description: String,
        weather_low_temperature: i32,
        weather_current_temperature: i32,
        weather_high_temperature: i32,
        weather_feels_like: i32,
        weather_wind_speed: f64, //meters/sec
        weather_wind_direction: String, //Meteorological degrees //0 from north blowing south, 90 from east blowing west
    }

    impl Weather {
    pub fn new(open_weather_response: &str) -> Self {
        Self {
            weather_code: parse_weather_code(open_weather_response),
            weather_description: parse_description(open_weather_response).to_string(),
            weather_low_temperature: parse_low_temperature(open_weather_response),
            weather_current_temperature: parse_current_temperature(open_weather_response),
            weather_high_temperature: parse_high_temperature(open_weather_response),
            weather_feels_like: parse_feels_like(open_weather_response),

            weather_wind_speed: parse_wind_speed(open_weather_response),
            weather_wind_direction: parse_wind_direction(open_weather_response),
        }
    }
}

    pub fn to_string(weather: &Weather) -> String {
    format!(
        "{}°F {} with low {}'s and high {}'s. Feels like: {}. Wind speeds at {}mph blowing: {}",
        //weather.weather_code,
        weather.weather_current_temperature,
        weather.weather_description,
        weather.weather_low_temperature,
        weather.weather_high_temperature,
        weather.weather_feels_like,
        weather.weather_wind_speed,
        weather.weather_wind_direction,
    )
}

//Parses weather code from openweather api response.
fn parse_weather_code(open_weather_response: &str) -> i32 {
    //regex101.com
    //Non-captures (?:) "id": and ,"main"
    //Captures ([0-9]*) i.e. the weather code https://openweathermap.org/weather-conditions#Weather-Condition-Codes-2
    let regular = Regex::new(r#"(?:"id":([0-9]*),"main")"#).unwrap();

    return parse_number_helper_int(regular, open_weather_response);

}

pub fn get_weather_code(weather_object: &Weather) -> i32 {
    return weather_object.weather_code;
}

fn parse_description(open_weather_response: &str) -> &str {
    let weather_code = parse_weather_code(open_weather_response);
    
    match weather_code {

        200 => "Thunderstorm with light rain",
        201 => "Thunderstorm with rain", 	
        202 => "Thunderstorm with heavy rain",
        210 => "Light thunderstorm",
        211 => "Thunderstorm",
        212 => "Heavy thunderstorm",
        221 => "Ragged thunderstorm",
        230 => "Thunderstorm with light drizzle",
        231 => "Thunderstorm with drizzle",
        232 => "Thunderstorm with heavy drizzle",

        300 => "Light drizzle",
        301 => "Drizzle",
        302 => "Heavy drizzle",
        310 => "Light drizzle/rain",
        311 => "Drizzle rain",
        312 => "Heavy drizzle/rain",
        313 => "Intermittent rain and drizzle",
        314 => "Heavy intermittent rain and drizzle",
        321 => "Intermittent drizzle",

        500 => "Light rain",
        501 => "Moderate rain",
        502 => "Heavy rain",
        503 => "Very heavy rain",
        504 => "Extreme rain",
        511 => "Freezing rain",
        520 => "Light showers",
        521 => "Medium showers",
        522 => "Heavy showers",
        531 => "Ragged showers",

        600 => "Light snow",
        601 => "Snow",
        602 => "Heavy snow",
        611 => "Sleet",
        612 => "Light sleet showers",
        613 => "Sleet showers",
        615 => "Light rain and snow",
        616 => "Rain and snow",
        620 => "Light snow showers",
        621 => "Snow showers",
        622 => "Heavy snow showers",

        701 => "Misty",
        711 => "Smokey",
        721 => "Hazey",
        731 => "Sand or dust whirls",
        741 => "Foggy",
        751 => "Sandy",
        761 => "Dusty",
        762 => "Volcanic ash",
        771 => "Squalls",
        781 => "Tornados",

        800 => "Clear skies",

        801 => "Some clouds: 11-25% cloud coverage",

        802 => "Scattered clouds: 25-50% cloud coverage",

        803 => "Broken clouds: 51-84% cloud coverage",

        804 => "Overcast clouds: 85-100% cloud coverage",

        _ => "Unknown weather",
    }
}

fn parse_low_temperature(open_weather_response: &str) -> i32 {
    let regular = Regex::new(r#"(?:temp_min":([0-9.]*),)"#).unwrap();
    return parse_number_helper_float(regular, open_weather_response).round() as i32;
}

fn parse_current_temperature(open_weather_response: &str) -> i32 {
    let regular = Regex::new(r#"(?:temp":([0-9.]*),)"#).unwrap();
    return parse_number_helper_float(regular, open_weather_response).round() as i32;
}

fn parse_high_temperature(open_weather_response: &str) -> i32 {
    let regular = Regex::new(r#""temp_max":([0-9.]*)"#).unwrap();
    return parse_number_helper_float(regular, open_weather_response).round() as i32;
}

fn parse_feels_like(open_weather_response: &str) -> i32 {
    let regular = Regex::new(r#"feels_like":([0-9.]*)"#).unwrap();
    return parse_number_helper_float(regular, open_weather_response).round() as i32;
}

fn parse_wind_speed(open_weather_response: &str) -> f64 {
    let regular = Regex::new(r#"speed":([0-9.]*)"#).unwrap();
    return parse_number_helper_float(regular, open_weather_response);
}

// {{"coord":{"lon":-1,"lat":39.7},"weather":[{"id":804,"main":"Clouds","description":"overcast clouds","icon":"04n"}],"base":"stations",
// "main":{"temp":47.23,"feels_like":47.23,"temp_min":47.23,"temp_max":47.23,"pressure":1017,"humidity":73,"sea_level":1017,"grnd_level":934},
// "visibility":10000,"wind":{"speed":2.93,"deg":82,"gust":2.53},"clouds":{"all":100},"dt":1762891809,"sys":{"country":"ES","sunrise":1762843440,
// "sunset":1762879932},"timezone":3600,"id":2520505,"name":"Calles","cod":200}
fn parse_wind_direction(open_weather_response: &str) -> String {
    let regular = Regex::new(r#"deg":([0-9.]*)"#).unwrap();
    let direction = parse_number_helper_float(regular, open_weather_response).round() as i32;
    return parse_float_to_direction(direction);
} 

//parse latitude and longitude
//need this for us states as the geocoding api only returns the coordinates, not the weather
//1 call fetch_weather_csc
//2 parse coordinates into floats
//3 send coordinates to fetch_weather_lat_long() which is the return of fetch_weather_csc()

pub fn parse_latitude(city_state_country_response: &str) -> f64 {
    let regular = Regex::new(r#"lat":([0-9.]*)"#).unwrap();
    let raw_float = parse_number_helper_float(regular, city_state_country_response);
    return round_to_hundreths_place(raw_float);

}

pub fn parse_longitude(city_state_country_response: &str) -> f64 {
    let regular = Regex::new(r#"lon":([0-9.]*)"#).unwrap();
    let raw_float = parse_number_helper_float(regular, city_state_country_response);
    return round_to_hundreths_place(raw_float);
}

fn round_to_hundreths_place(coordinate: f64) -> f64 {
    return (coordinate * 100.0).round() / 100.0;
}


//helper functions

fn parse_number_helper_int(reg: Regex, open_weather_response: &str) -> i32 {

    if let Some(caps) = reg.captures(open_weather_response) {
        let wr = &caps[1];

        if let Ok(wc) = wr.parse::<i32>() {
        return wc;
        }
        else {
            return -1;
        }
        return -1;
    }
    return -1;
}

fn parse_number_helper_float(reg: Regex, open_weather_response: &str) -> f64 {

    if let Some(caps) = reg.captures(open_weather_response) {
        let wr = &caps[1];

        if let Ok(wc) = wr.parse::<f64>() {
        return wc;
        }
        else {
            return -1.0;
        }
        return -1.0;
    }
    return -1.0;
}

fn parse_float_to_direction(int_direction: i32) -> String {
    let mut direction: String = String::new();
    match int_direction {
        
        348..360 => direction = String::from("South"),

        0_i32..=12_i32 => direction = String::from("South"),
        13_i32..=34_i32 => direction = String::from("South-Southwest"),
        35_i32..=56_i32 => direction = String::from("Southwest"),
        57_i32..=79_i32 => direction = String::from("West-Southwest"),
        80_i32..=101_i32 => direction = String::from("West"),

        102_i32..=124_i32 => direction = String::from("West-Northwest"),
        125_i32..=146_i32 => direction = String::from("Northwest"),
        147_i32..=169_i32 => direction = String::from("North-Northwest"),
        170_i32..=191_i32 => direction = String::from("North"),

        192_i32..=214_i32 => direction = String::from("North-Northeast"),
        215_i32..=236_i32 => direction = String::from("Northeast"),
        237_i32..=259_i32 => direction = String::from("East"),
        260_i32..=281_i32 => direction = String::from("East-Southeast"),

        282_i32..=304_i32 => direction = String::from("Southeast"),
        305_i32..326_i32 => direction = String::from("South-Southeast"),

        _ => direction = String::from("Unknown direction"),
    }
    return direction;
}

fn parse_helper_string(reg: Regex, open_weather_response: &str) -> String {
    if let Some(caps) = reg.captures(open_weather_response) {
        return caps //Throws an ownership error which was really annoying.
        //Fix
        .get(1) //Gets first capture group
        .map(|m| m.as_str().to_string()) //Borrows substring of the m, match object
        .unwrap_or("Error".to_string()); //Owned string
    }
    return "Error".to_string();
}


