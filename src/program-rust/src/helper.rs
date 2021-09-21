use solana_program::{
    msg,
};

use sha2::Sha512;
use hmac::{Hmac, Mac, NewMac};
type HmacSHA512 = Hmac<Sha512>;

//utilities
pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strs.join("")
}

pub fn hash_to_number(slice : &str) -> i64{
    let z = i64::from_str_radix(slice, 16);
    z.unwrap()
}

//main fns
pub fn convert_to_hmacsha512(server_seed : String, input: String, nonce: u8) -> String{
    let mut mac = HmacSHA512::new_from_slice(server_seed.as_bytes()).expect("some error occured");
    mac.update(&format!("{} - {}", input.trim().to_string(), nonce).into_bytes());
    let result = mac.finalize();
    let byte_arr = result.into_bytes();
    to_hex_string(&byte_arr.to_vec())
}

pub fn validate_hex_characters(hmac: &String) -> Option<i64> {
    let mut start: usize = 0;
    let mut end: usize = 5;
    let mut slice1 = &hmac[start..end];

    loop {
        let no = hash_to_number(slice1);

        if no < 999999{
            return Some(no);
        } else {
           println!("Need for increment");
           start += 5;
           end += 5;
           if end > 127 {
                println!("Hash length exceeds {}", end);
                return None;
           }
           slice1 = &hmac[start..end];
        }
    }
}

pub fn index(range: usize, rndm: f64) -> usize{
    // range (max - min) * (n, 1) + min;
    let idx = (range - 1) as f64 * rndm;
    idx as usize
}

pub fn generate_random_number(client_seed: &str, nonce: u8, vec_len: u32) -> u32 {
    let server_seed = "S0m3S3rv3rs33d1337";
    // let mut main_vec = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "x", "y", "z"];
    // let mut bundle_vec = Vec::new();
     
    let hmachash: String = convert_to_hmacsha512(server_seed.to_string(), client_seed.to_string(), nonce);
    msg!("hmachash: {}", hmachash);
    let rnd = validate_hex_characters(&hmachash).unwrap()%(10000)/100;

    msg!("rnd: {}", rnd);
    
    let mut frnd = rnd as f64;

    msg!("frnd: {}", frnd);

    if frnd < 10.0{
        frnd = frnd + 10.0;
    }

    frnd = frnd/100.0;
    frnd = (frnd * 10.0) / 10.0;

    let idx = index(vec_len as usize, frnd);
    return idx as u32;   
}