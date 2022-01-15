/*
This file contains the logic for swedish person number generation
*/

// Imports
use rand::{Rng, random};

pub fn generate_pn() -> f32{
    /* 
    A function that generates swedish personal numbers based on the following formula:
    
    1. Random date between 1950 and 18 year earlier from now
    date(year(4)-month(2)-day(2))
    2. Random 3 digits
    3. Last digit following the checksum calculation in 
    https://en.wikipedia.org/wiki/Personal_identity_number_(Sweden)#Checksum
    */
    // Generating the date


    // Generating the first 3 digits in the PN code
    let mut rng = rand::thread_rng();
    let frst_three_int: i32 = rng.gen_range(0..999);
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
    1.1
}