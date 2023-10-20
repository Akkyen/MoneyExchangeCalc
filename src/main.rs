// Importing necessary libraries and modules
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;
use std::io::{Read, stdin, stdout, Write};
use std::process::exit;

// Struct definition for deserializing the JSON data from the currency API
#[derive(Debug, Serialize, Deserialize)]
struct CurrencyJsonDataset {
    // Date of the currency data
    pub date: NaiveDate,
    // Mapping of currency codes to their exchange rate against EUR
    pub eur: HashMap<String, f32>,
}

fn main() -> Result<()> {
    // Fetching the latest currency data for EUR from the API and getting it as a string
    let body = reqwest::blocking::get("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/eur.json").unwrap().text().unwrap();

    // Parsing the JSON data into a CurrencyJsonDataset struct
    let data = currency_json_parsing(&body)?;

    // Main loop for user interaction
    loop {
        let mut value = 1.0;
        let mut value_str = String::new();
        let mut code = String::new();

        println!("Mit der Eingabe von exit kann das Programm jederzeit beendet werden.");
        println!("Die verwendeten Kurse sind vom {} Uhrzeit unbekannt!", data.date.format("%d.%m.%Y"));

        loop {
            println!();
            // Prompting user for the amount in EUR they want to convert
            println!("Bitte gibt die Menge an Euro ein, die du konvertieren möchtest: ");

            value_str = String::new();

            // Reading user input
            if stdin().read_line(&mut value_str).is_ok()
            {
                value_str = value_str.trim().parse().unwrap();

                exit_user(&*value_str);

                // Parsing the user input to a floating point number
                value = match value_str.parse() {
                    Ok(v) => v,
                    Err(_) => {
                        println!("Dies ist kein Integer oder Float!");
                        continue;
                    }
                };

                break;
            } else {}
        }

        // Prompting user for the target currency code

        loop {
            println!();
            println!("Bitte gibt den Code der Währung ein in die konvertiert werden soll: ");

            code = String::new();

            // Reading the currency code from the user
            match stdin().read_line(&mut code) {
                Ok(_) => {
                    code = code.trim().parse().unwrap();

                    if data.eur.contains_key(&code) {
                        break;
                    }

                    exit_user(&*value_str);
                },
                Err(_) => {},
            }
        }
        println!();

        // Displaying the conversion result to the user
        println!("Du erhälst für {} Euro {} {}!", value_str, value * data.eur[&code], code);
        pause();

        // Clearing the console screen
        print!("{}[2J", 27 as char);
        stdin().read_line(&mut value_str);
    }
}

// Function to parse the JSON string into a CurrencyJsonDataset struct
fn currency_json_parsing(data: &str) -> Result<Box<CurrencyJsonDataset>> {
    let cur_data_set: CurrencyJsonDataset = serde_json::from_str(data)?;
    Ok(Box::new(cur_data_set))
}

// Function to pause the console until the user presses a key
fn pause() {
    write!(stdout(), "Drücken Sie eine beliebige Taste, um fortzufahren...").unwrap();
    stdout().flush().unwrap();
    let _ = stdin().read(&mut [0u8]).unwrap();
}

fn exit_user(value_str: &str) {
    if value_str.eq_ignore_ascii_case("exit") {
        // Exiting the program if the user types "exit"
        exit(0);
    }
}
