/*
This file contains the logic for swedish person number generation
*/

// Imports
use rand::{Rng};
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
    let curr_timestamp = chrono::Local::now();
    let curr_time = curr_timestamp.time();
    let curr_date = curr_timestamp.date();

    let time_date = NaiveDate::from_ymd(curr_date.year()-age, curr_date.month(),
                                                    curr_date.day()).and_hms(curr_time.hour(),
                                                    curr_time.minute(), curr_time.second());

    let mut rng = rand::thread_rng();
    let rnd_tstamp: i64 = rng.gen_range(0..time_date.timestamp());
    let rnd_date = NaiveDateTime::from_timestamp(rnd_tstamp, 0);
    let mut rnd_formatted_date = rnd_date.format("%Y%m%d").to_string();

    // Generating the first 3 digits in the PN code
    let frst_three_int: i32 = rng.gen_range(0..1000); //random number between 0 and 999
    let mut frst_three_str: String = frst_three_int.to_string();

    match frst_three_str.len() {
        1 => {
            let filler = "00";
            frst_three_str = [filler, &frst_three_str].concat();
        },
        2 => {
            let filler = "0";
            frst_three_str = [filler, &frst_three_str].concat();
        },
        _ => ()
    }


    // Generating the last digit in the PN code
    let pn_known = [rnd_formatted_date.clone(), frst_three_str.clone()].concat();
    let mut counter = 0;
    let mut value: u32 = 0;
    let mut processed_value = 0;
    for char in pn_known[2..].chars() {
        counter += 1;
        if counter%2!=0{
            value = 2 * char.to_digit(10).unwrap();
            if value > 9{
                let str_value = value.to_string();
                let mut sub_sum = 0;
                for sub_char in str_value.chars() {
                    sub_sum += sub_char.to_digit(10).unwrap();
                }
                processed_value += sub_sum;
                continue;
            }
            processed_value += value;
            continue;
        }
        processed_value += char.to_digit(10).unwrap();
        continue;
    }

    let checksum_value = 10-processed_value.to_string().chars().last().unwrap().to_digit(10).unwrap();
    rnd_formatted_date.push_str("-");
    rnd_formatted_date.push_str(&frst_three_str);
    rnd_formatted_date.push_str(&checksum_value.to_string());


    rnd_formatted_date
}