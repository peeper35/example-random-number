use solana_program::{
    msg,
};

use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};
type HmacSHA256 = Hmac<Sha256>;

// utilities, convert byte array to hex string
pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    strs.join("")
}

// convert string hex to number
pub fn hash_to_number(slice : &str) -> i64{
    let z = i64::from_str_radix(slice, 16);
    z.unwrap()
}

// generate hmac from clientseed, serverseed and nonce
pub fn convert_to_hmac_sha256(server_seed : String, input: String, nonce: u8) -> String{
    let mut mac = HmacSHA256::new_from_slice(server_seed.as_bytes()).expect("some error occured");
    mac.update(&format!("{} - {}", input.trim().to_string(), nonce).into_bytes());
    let result = mac.finalize();
    let byte_arr = result.into_bytes();
    to_hex_string(&byte_arr.to_vec())
}

// validate first hex 5 hex chars if they are lesser than int 999999
// if not shift to next 5 and check and vice versa
// return number for 5 chars if those 5 chars hex is lesser than 999999 
pub fn validate_hex_characters(hmac: &String) -> Option<i64> {
    let mut start: usize = 0;
    let mut end: usize = 5;
    let mut slice1 = &hmac[start..end];

    loop {
        let no = hash_to_number(slice1);

        if no < 999999{
            return Some(no);
        } else {
           msg!("Need for increment");
           start += 5;
           end += 5;
           if end > 63 {
                msg!("Hash length exceeds {}", end);
                return None;
           }
           slice1 = &hmac[start..end];
        }
    }
}

// fetching the index
pub fn index(range: usize, rndm: f64) -> usize {
    let idx = (range - 1) as f64 * rndm;
    idx as usize
}

// main function 
pub fn generate_random_number(client_seed: &str, nonce: u8, vec_len: u32) -> u32 {
    let server_seed = "S0m3S3rv3rs33d1337";
     
    let hmachash: String = convert_to_hmac_sha256(server_seed.to_string(), client_seed.to_string(), nonce);
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