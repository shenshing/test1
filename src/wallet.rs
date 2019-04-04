
extern crate v_1;
use v_1::vec_to_str;
use v_1::hash;
use v_1::Hash;
use crate::block;
use crate::block::Block;
use block::Transaction;
use block::fn_transaction;
pub type list_block<'a> = &'a Vec<Block<'a>>;
use std::io;
pub struct Wallet<'a>{
    pub account:     String,
    pub password:    String,
    pub wallet_id:   String,
    pub block:       list_block<'a>,
    pub balance:     f64,
}
impl<'a> Wallet<'a>{
    pub fn new(vec: &'a Vec<Block>) -> Self{
        Wallet{
                account:    "null".to_string(),
                password:   "null".to_string(),
                block:       vec,
                wallet_id: "null".to_string(),
                balance:    00.00,
            }
    }
    pub fn create_wallet(vec: &'a Vec<Block>) -> Self{
        let mut acc = String::new();
        let mut pw  = String::new();
        let mut ha = String::new();

        println!("input account:    ");
        io::stdin().read_line(&mut acc).expect("Invalid input");
        println!("input password:   ");
        io::stdin().read_line(&mut pw).expect("Invalid input");
        let mut st = String::new();
        st = acc.clone() + &pw;
        ha = hash(&st);  //error
            Wallet{
                    account:    acc,
                    password:   pw,
                    block:      &vec,  
                    wallet_id: ha,
                    balance:    0.00,
            }
    }
    pub fn show_wallet(&self){
        println!("Account:      {}",&self.account.trim());
        println!("Password:     {}",&self.password.trim());
        println!("Wallet ID:    {}",&self.wallet_id);
        println!("Balance:      $ {}",&self.balance);
    }
    

}
pub fn output(vec: &Vec<Wallet>){
    println!("*****ACCOUNT LIST*****");
        for i in 0..vec.len(){
            vec[i].show_wallet();
            println!("----------------");
        }
}

pub struct Valid{
    pub address:    String,
    pub amount:     f64,
}
impl Valid{
    pub fn new(address: String, amount: f64) -> Valid{
        Valid{
            address,
            amount,
        }
    }
    pub fn show(&self){
        println!("  Address:  {}",&self.address);
        println!("  Amount:   {}",&self.amount);
    }
    
}


pub fn show_valid(vec: &Vec<Valid>){
    for i in 0..vec.len(){
        vec[i].show();
    }
}
fn main(){

}
