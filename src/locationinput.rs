//use std::io;
use std::io::{self, Write};

//pub mod util;
pub use crate::util::utility::{greet, sec_sleep, clear_screen};

//i wanted to tie this up into a nice neat object to encapsulate the class functions. but this optional 'state' field is too much trouble.
pub struct Location {
        city: String,
        //State field is tricky. It may or may not exist. Exists IFF the country is the US.
        state: Option<String>,
        country: String,
    }

//     impl Location {
//     pub fn new(open_weather_response: &str) -> Self {
//         Self {
//             city:// ,
//             state: //,
//             country://,
//         }
//     }
// }

pub fn user_country() -> String {
    loop {
        print!("Enter country: ");
        io::stdout().flush().unwrap();

        let mut entered_country = String::new();
        io::stdin()
            .read_line(&mut entered_country)
            .expect("bad country input");


        let cleaned: String = entered_country
            .trim() //remove leading and trailing whitespaces
            //.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace()) //characters and spaces only
            .collect();

        if !cleaned.is_empty() {
            return cleaned;
        } else {
            println!("Invalid country");
        }
    }
}

//user_country()'s helper function for special case country: united states.
pub fn is_united_states(country: &str) -> bool {
    let lower_case_country = country.to_lowercase();
    match lower_case_country.as_str() {
        "united states" 
        | "us" 
        | "usa" 
        | "united states of america" => true,
        _ => false,
    }
}

//yea forgive me for not checking against all fifty states and their abbreviations
pub fn user_state() -> String {
    loop {
        print!("Enter state: ");
        io::stdout().flush().unwrap();

        let mut entered_state = String::new();
        io::stdin()
            .read_line(&mut entered_state)
            .expect("bad state input");

        // lowercase and remove non letters or space characters
        let cleaned: String = entered_state
            .trim()
            //.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect();

        if !cleaned.is_empty() {
            return cleaned;
        } else {
            println!("Invalid state.");
        }
    }
}

pub fn user_city() -> String {
    loop {
        print!("Enter city: ");
        io::stdout().flush().unwrap();

        let mut entered_city = String::new();
        io::stdin()
            .read_line(&mut entered_city)
            .expect("bad city input");

        let cleaned: String = entered_city
            .trim()
            //.to_lowercase()
            .chars()
            .filter(|c| c.is_ascii_alphabetic() || c.is_whitespace())
            .collect();

        if !cleaned.is_empty() {
            return cleaned;
        } else {
            println!("Invalid City.");
        }
    }
}




