use std::collections::HashMap;
use std::io::{stdin};

// Importing necessary libraries and modules
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Result;

mod utils;

// Struct definition for deserializing the JSON data from the currency API
#[derive(Debug, Deserialize)]
pub struct CurrencyJsonDataset {
	// Date of the currency data
	pub date: NaiveDate,
	// Mapping of currency codes to their exchange rate against EUR
	#[serde(flatten)]
	pub exchange: HashMap<String, HashMap<String, f32>>,
}

fn main() -> Result<()> {
	// Main loop
	loop {
		let mut amount_cur_source: f32;

		let mut cur_code_source : String;
		let mut cur_code_target : String;


		// Gets a map of all supported currencies.
		// The map goes cur_code -> cur_full_name
		let currencies: HashMap<String, String> = serde_json::from_str(
			&*reqwest::blocking::get("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies.json")
				.unwrap().text().unwrap()).
			expect("ERROR: Either there is no JSON provided at the used URL or the JSON could \
			 not get serialized to the internal representation!");


		println!("With the input of \"exit\", the program can be terminated at any time.");


		loop {
			cur_code_source = utils::cli_read_line("Please enter the code of the currency you want to use: ").1;

			utils::check_for_exit_prompt(&cur_code_source);

			if currencies.contains_key(&cur_code_source) {
				break
			}

			println!();
		}


		// Getting the currency conversion modifiers for cur_code_source
		let api_url = format!("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{}.json", &cur_code_source);
		let data = utils::currency_json_parsing(&*reqwest::blocking::get(api_url).unwrap().text().unwrap()).unwrap();


		println!("The exchange rates used are from {} (time unknown)!", data.date.format("%d.%m.%Y"));
		println!();


		//To be converted Euro Amount - Loop
		loop {
			let msg = format!("Please enter the amount of {} you want to convert: ", currencies.get(&*cur_code_source).unwrap());

			let amount_as_str = utils::cli_read_line(&*msg).1;

			println!();

			utils::check_for_exit_prompt(&*amount_as_str);

			amount_cur_source = match amount_as_str.parse() {
				Ok(v) => v,
				Err(_) => {
					println!("This is not an integer or float!");

					println!();

					continue;
				}
			};

			if amount_cur_source < 0.0 {
				continue;
			}

			break;
		}


		// Currency Code - Loop
		loop {
			let msg = "Please enter the code of the currency you want to convert to: ";

			cur_code_target = utils::cli_read_line(msg).1;

			println!();

			utils::check_for_exit_prompt(&cur_code_target);

			if currencies.contains_key(&cur_code_target) {
				break;
			}
		}


		// Displaying the conversion result to the user
		println!("You will receive {} {} for {} {}!",
		         amount_cur_source * data.exchange.get(&cur_code_source).unwrap()[&cur_code_target],
		         currencies[&cur_code_target],
		         amount_cur_source,
		         currencies[&cur_code_source]);


		utils::cli_pause();


		// Clearing the console screen
		print!("{}[2J", 27 as char);
		let _ = stdin().read_line(&mut "".to_string());
	}
}