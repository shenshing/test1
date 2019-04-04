// mod block;
// use block::Block;
// use block::BlockChain;
// use block::current_time;
// use block::block__chain;

// use block::Transaction;

// mod person;
// use person::Person;

// pub mod wallet;
// extern crate v_1;
// mod wallet;
// use wallet::Wallet;

// use chrono::{DateTime,Utc};
// pub type Transact = Vec<Transaction>;
// use v_1::Transaction;
// use block::Transaction;
// extern crate v_1;
// use v_1::Wallet::create_wallet;
// mod wallet;

// pub struct Wallet{
//     pub account:    String,
//     pub password:   String,
//     pub wallet_id:  Hash,
//     pub balance:    f64,
// }

extern crate crypto;
extern crate rand;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

extern crate rustc_serialize;
use crypto::symmetriccipher::SynchronousStreamCipher;
use std::iter::repeat;

//CIPHERs
use crypto::aes::{self,KeySize};
use rand::RngCore;
use rand::OsRng;
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::STANDARD;

pub type Hash = Vec<u8>;
// pub type Hash = ;
// pub fn hash (data: &String) -> Hash {
//     crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
// }

pub fn hash(data: &String) -> String{
    // let mut st = String::new();
    let mut sha = Sha256::new();
    sha.input_str(data);
    sha.result_str()
}

// pub fn vec_to_str(vector: &Vec<u8>) -> String{
//     let mut st = String::new();
//     st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
//     st
// }
pub fn vec_to_str(vector: &Vec<u8>) -> String{
    vector.to_base64(STANDARD)
}

// use crate::block::Transaction;
// use block;
// use block::Transaction;

use std::collections::HashMap;
fn remove_hashmap(x: &mut HashMap<i32, String>,remove: i32) {
    let tmp = x.clone();
    let empties = tmp
         .iter()
         .filter(|&(&v, _)| v == remove)
         .map(|(k, _)| k);

    for k in empties { x.remove(k); }
}

pub fn vec_to_str1(vec: Vec<i32>) -> String{
    let mut st = String::new();
    for i in vec.iter(){
        st = st + &i.to_string();
    }
    st
}
// pub fn current_time() -> String{
//         let now: DateTime<Utc> = Utc::now();
//         let mut time = String::new();
//         time = now.format("%a %e %b %Y %T.%f").to_string();
//         time
// }

