/*
This file contains the logic for swedish person number generation
*/

// Imports
use rand::Rng;
use chrono::{NaiveDate, NaiveDateTime, Datelike, Timelike};

pub fn generate_pn(age: i32) -> String{
    /*
    A function that generates swedish personal numbers based on the following formula:

    1. Random date between 1950 and 18 year earlier from now
    date(year(4)-month(2)-day(2))
    2. Random 3 digits
    3. Last digit following the checksum calculation in
    https://en.wikipedia.org/wiki/Personal_identity_number_(Sweden)#Checksum
    */

    // Generating the date
    let current_timestamp = chrono::Local::now();
    let current_time = current_timestamp.time();
    let current_date = current_timestamp.date();

    let date_max = NaiveDate::from_ymd(current_date.year()-age, current_date.month(),
                                                    current_date.day()).and_hms(current_time.hour(),
                                                    current_time.minute(), current_time.second());

    let mut random_number_generator = rand::thread_rng();
    let random_timestamp: i64 = random_number_generator.gen_range(0..date_max.timestamp());

    let random_date = NaiveDateTime::from_timestamp(random_timestamp, 0);
    let mut random_formatted_date = random_date.format("%Y%m%d").to_string();

    // Generating the first 3 digits in the PN code

    let mut leading_ints = random_number_generator.gen_range(0..1000).to_string();

    match leading_ints.len() {
        1 => {
            leading_ints = ["00", &leading_ints].concat();
        },
        2 => {
            leading_ints = ["0", &leading_ints].concat();
        },
        _ => ()
    }


    // Generating the last digit in the PN code
    let pn_known = format!("{}{}",random_formatted_date,leading_ints);
    let mut counter = 0;
    let mut processed_value = 0;
    for char in pn_known[2..].chars() {
        counter += 1;
        if counter%2!=0{
            let value = 2 * char.to_digit(10).unwrap();
            if value > 9{
                let mut sum_of_numbers = 0;
                for number in value.to_string().chars() {
                    sum_of_numbers += number.to_digit(10).unwrap();
                }
                processed_value += sum_of_numbers;
                continue;
            }
            processed_value += value;
            continue;
        }
        processed_value += char.to_digit(10).unwrap();
        continue;
    }

    // Calculatin checksum
    let mut checksum = 10-(processed_value % 10);
    if checksum == 10 {
        checksum = 0;
    }

    random_formatted_date.push_str("-");
    random_formatted_date.push_str(&leading_ints);
    random_formatted_date.push_str(&checksum.to_string());


    random_formatted_date
}