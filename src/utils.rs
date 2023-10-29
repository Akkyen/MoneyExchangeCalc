use std::io::{Read, stdin, stdout, Write};
use std::process::exit;
use crate::CurrencyJsonDataset;

/// Function to parse the JSON string into a CurrencyJsonDataset struct
pub fn currency_json_parsing(data: &str) -> Result<Box<CurrencyJsonDataset>, serde_json::Error>
{
	let cur_data_set: CurrencyJsonDataset = serde_json::from_str(data)?;
	Ok(Box::new(cur_data_set))
}

/// Pauses the program execution and waits for the user to press any key.
///
/// This function displays a message "Press any key to continue..." and then waits for a single key press from the user.
/// The program execution resumes once any key is pressed.
pub fn cli_pause() {
	// Display the message without a newline and ensure it's immediately displayed to the user
	write!(stdout(), "Press any key to continue...").unwrap();
	stdout().flush().unwrap();

	// Wait for a single key press from the user.
	// The key press is read into a byte array but is not used.
	let _ = stdin().read(&mut [0u8]).unwrap();
}

/// Reads a line from the user and returns a tuple containing the length of the input and the input itself.
///
/// # Arguments
///
/// * `msg` - A message to display prompting the user for input.
///
/// # Returns
///
/// Returns a tuple where the first element is the length of the input string and the second element is the input string.
///
/// # Panics
///
/// This function will panic if the trimming of the input string results in a format that cannot be parsed into a `String`.
pub fn cli_read_line(msg: &str) -> (usize, String) {
	// Create an empty mutable String to hold the user's input
	let mut input_str = String::new();

	// Infinite loop to repeatedly prompt the user until valid input is received
	loop {
		// Display the prompt message to the user
		println!("{}", msg);

		// Attempt to read a line from stdin into the input_str
		match stdin().read_line(&mut input_str) {
			// If the read is successful...
			Ok(_) => {
				// Trim whitespace from the input and parse it into a String
				input_str = input_str.trim().parse().unwrap();

				// Return a tuple containing the length of the trimmed input string and the trimmed string itself
				return (input_str.len(), input_str);
			},
			// If there's an error reading from stdin, continue with the next iteration of the loop
			Err(_) => continue,
		};
	}
}

pub fn check_for_exit_prompt(value_str: &str) {
	if value_str.eq_ignore_ascii_case("exit") {
		// Exiting the program if the user types "exit"
		exit(0);
	}
}