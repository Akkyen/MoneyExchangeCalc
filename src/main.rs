use io::stdout;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io;
use std::io::{Read, stdin, Write};
use std::process::exit;

#[derive(Debug, Serialize, Deserialize)]
struct CurrencyJsonDataset {
    pub date: NaiveDate,
    pub eur: HashMap<String, f32>,
}

fn main() -> Result<()>
{
    let body = reqwest::blocking::get("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/eur.json").unwrap().text().unwrap();

    let data = currency_json_parsing(&*body)?;

    loop
    {
        let mut value = 1.0;

        let mut value_str = String::new();

        let mut code = String::new();

        println!("Mit der Eingabe von exit kann das Programm jederzeit beendet werden.");
        println!("Die verwendeten Kurse sind vom {} Uhrzeit unbekannt!", data.date.format("%d.%m.%Y"));
        println!();

        println!("Bitte gibt die Menge an Euro ein, die du konvertieren möchtest: ");

        loop {
            value_str = String::new();

            if match std::io::stdin().read_line(&mut value_str) {
                Ok(_v) => {
                    value_str = value_str.trim().parse().unwrap();
                    true},
                Err(_e) => {
                    false
                },
            }
            {
                if value_str.eq_ignore_ascii_case("exit") {
                    exit(0);
                }

                value = match value_str.parse() {
                    Ok(v) => v,
                    Err(_e) => {
                        println!("Dies ist kein Integer oder Float!");
                        println!();
                        println!("Bitte gibt die Menge an Euro ein, die du konvertieren möchtest: ");
                        continue
                    }
                };

                break
            }
            else
            {
                println!();
                println!("Bitte gibt die Menge an Euro ein, die du konvertieren möchtest: ");
            }

        }


        println!("Bitte gibt den Code der Währung ein in die konvertiert werden soll: ");

        loop {
            code = String::new();

            match stdin().read_line(&mut code) {
                Ok(_v) => {

                    code = code.trim().parse().unwrap();

                    if data.eur.contains_key(&*code)
                    {
                        break
                    }

                    if code.eq_ignore_ascii_case("exit") {
                        exit(0);
                    }

                    println!()
                },
                Err(_e) => {
                    println!()
                },
            }
        }



        println!("Du erhälst für {} Euro {} {}!", value_str, value * data.eur[&code], code);
        pause();
        print!("{}[2J", 27 as char);
        stdin().read_line(&mut value_str);
    }
}

fn currency_json_parsing(data: &str) -> Result<Box<CurrencyJsonDataset>>
{
    let cur_data_set: CurrencyJsonDataset  = serde_json::from_str(data)?;
    Ok(Box::new(cur_data_set))
}

fn pause()
{
    write!(stdout(), "Drücken Sie eine beliebige Taste, um fortzufahren...").unwrap();
    stdout().flush().unwrap();

    let _ = stdin().read(&mut [0u8]).unwrap();
}