# MoneyExchangeCalc

This program allows users to convert Euros (EUR) to a desired currency using the exchange rate fetched from a currency API. 

### Features:

1. Fetch currency data for EUR from an API.
2. Convert a given amount in EUR to a target currency.
3. Clear and user-friendly console-based interface.
4. Ability to exit the program at any point by typing "exit".

### Dependencies:

- **chrono**: For date handling.
- **serde**: For serializing and deserializing the JSON data.
- **serde_json**: For parsing JSON results.
- **reqwest**: For making HTTP requests.
- **std**: Standard library for collections, I/O operations, and process control.

### Structs:

- **CurrencyJsonDataset**: Represents the JSON data structure from the currency API.
  - `date`: Date of the currency data.
  - `eur`: Mapping of currency codes to their exchange rate against EUR.

### Main Functions:

1. **main()**: The main function where the program execution starts.
2. **currency_json_parsing(data: &str)**: Parses the JSON string into a `CurrencyJsonDataset` struct.
3. **pause()**: Pauses the console until the user presses a key.
4. **exit_user(value_str: &str)**: Checks if the user input is "exit" and exits the program if true.

### Usage:

Run the program. You will be prompted to:
1. Enter the amount in EUR you want to convert.
2. Enter the code of the target currency (e.g., USD, GBP, etc.).
3. The program will display the converted amount in the desired currency.
4. You can continue converting or type "exit" at any input prompt to terminate the program.

### Notes:
- The currency data source is "https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/eur.json".
- The program assumes that the API always provides valid and up-to-date currency data.

### Contributing:

Feel free to submit pull requests for new features or bug fixes.

### License:

EUPL v1.2
