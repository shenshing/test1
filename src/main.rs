
#[allow(dead_code)]
fn vector_loop(vector: &[String]){
    for value in vector{
        println!("{}",value);
    }
}

#[allow(dead_code)]
extern crate chrono;

use chrono::{DateTime,Utc};
use std::time::{SystemTime,UNIX_EPOCH};

#[allow(dead_code)]

mod block;
use block::BlockChain;
use block::Block;

use block::Transaction;
use block::fn_transaction; 
use block::struct_to_str;  
use block::current_time;
use block::block__chain;


use wallet::Wallet;

extern crate v_1;
use v_1::vec_to_str;
use v_1::hash;
use v_1::vec_to_str1;

mod wallet;
use std::str;
use std::io;
pub struct RemainCoin{
    total:  u32,
}
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use std::error::Error;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
fn main(){
    let mut path = Path::new("blockchain.txt");
    
    let mut total_bitcoin: f64 = 100.00;
    
    let mut combine_string = String::new();
    let mut wallet_list: Vec<Wallet>      = Vec::new();
    let mut blockchain:  Vec<Block>       = Vec::new();
    let mut tran_list:   Vec<Transaction> = Vec::new();

    
    let mut account = String::new();
    let mut password = String::new();
    loop{
        let mut input = String::new();
        let mut input1 = String::new();
        // input.clear();
        println!("\t1- Create new Wallet");
        println!("\t2- Show all wallet");
        println!("\t3- Make transaction");
        println!("\t4- Mining (just test)");
        println!("\t5- Log in");
        println!("\t6- Not yet confirm transaction");
        println!("<enter> your choice:  ");
        io::stdin().read_line(&mut input).unwrap();
            match input.trim().as_ref(){
                "1" => {
                        println!("You're going to create a new wallet");
                        let mut new_account = Wallet::create_wallet(&blockchain);
                        wallet_list.push(new_account);
                }
                "2" => {
                        println!("You're going to show all wallets");
                        wallet::output(&wallet_list);
                            println!("\t\tpress <s> to see blockchain");
                            println!("\t\tpress any key to leave");
                }
                "3" => println!("You're going to make transaction"),
                "4" => {
                        // input.clear();
                        let mut input = String::new();
                        input.clear();
                        let mut result = 0;
                        println!("You're going to mining");
                        let mut block = block::Block::genesis_block(&tran_list);
                        println!("Winner:   ");
                        io::stdin().read_line(&mut input).expect("ERROR: Invalid input");
                        // println!("You input {}",input.trim());
                            for i in 0..wallet_list.len(){
                                if wallet_list[i].account == input{
                                    result = 1;
                                    // wallet_list[i].balance += 12.0;
                                    println!("Money:    {}",wallet_list[i].balance);
                                    for i in 0..tran_list.len(){
                                        for i1 in 0..wallet_list.len(){
                                            if tran_list[i].sender_public_key == wallet_list[i1].wallet_id{
                                                println!("Found Account");
                                                if wallet_list[i1].balance > tran_list[i].amount{
                                                    println!("Sender has enough balance");
                                                    for i2 in 0..wallet_list.len() {
                                                        if tran_list[i].receiver_public_key == wallet_list[i2].wallet_id{
                                                            wallet_list[i1].balance -= tran_list[i].amount + 0.1;
                                                            let mut bonous = 0.1 * (tran_list.len() as f64);
                                                            wallet_list[i2].balance += tran_list[i].amount + 0.1;
                                                        }
                                                    }
                                                }else{
                                                    println!("Sender not enough balance");
                                                }
                                            }
                                        }//second loop
                                    }//first loop
                                }
                            }
                        if result == 0{
                            println!("SORRY No account found");
                        }else if result == 1{
                            println!("Mining Complete....");
                            
                            for i in 0..wallet_list.len(){
                                if wallet_list[i].account == input{
                                    wallet_list[i].balance += 12.0;
                                }
                            }
                            // let mut id1 = blockchain.len() as u32 + 1u32;
                            // let mut block = block::Block{
                            //     id:         id1,
                            //     timestamp:  current_time(),
                            //     hash:       hash(&struct_to_str(&tran_list)),
                            //     prev_hash:  hash(&struct_to_str(&tran_list)),
                            //     data:       &tran_list,
                            // };
                //////////////////////////////////////////
                    let mut file = match File::open(path){
                        Err(why) => panic!("ERROR:  {}",why.description()),
                        Ok(file) => file,
                    };
                    let mut id1 = String::new();
                    let mut p_hash = String::new();
                    // let mut id1 = String::from("1");
                    // let mut vec_hash = vec![0;64];
                    // p_hash = vec_to_str1(vec_hash);
                    let reader = BufReader::new(file);
                        for line in reader.lines(){
                            let line = line.unwrap();
                            // println!("----------");
                            // println!("{}",line);
                            // println!("----------");
                            let mut vec1 = vec![];
                            let mut vec2 = vec![];
                            // let mut vec3 = vec![];
                            for s in line.split(|x| (x == ',') || (x == '[') || (x == ']')){
                                if vec1.len() < 4{
                                    vec1.push(s);
                                }else{
                                    vec2.push(s);
                                }
                            }
                            // vec1.reverse();
                            println!("vec1[0]  =    {}",vec1[0]);
                            id1 = vec1[0].to_string();
                            p_hash = vec1[2].to_string();
                            // println!("{:#?}",vec1);
                        }
                    
                //////////////////////////////////////////            
                            let mut id2: u32 = id1.trim().parse().unwrap();
                            let mut id1: u32 = id2 + 1u32;
                            // let mut id1: u32 = id2;
                            let mut timestamp = current_time();
                            let mut hash_str = format!("{}{}{}",struct_to_str(&tran_list),id1.to_string(),timestamp);
                            let mut hash1 = hash(&hash_str);
                            let mut prev_hash = p_hash;
                            // let mut prev_hash = hash(&vec_to_str1(vec_hash));
                            let mut id1 = id1.to_string();
                println!("****************************id =  {}",id1);
                                // blockchain.push(block);
                                // tran_list.clear();
                                let mut st = String::new();
                                st.clear();
                                st = block::combine(&id1,&timestamp,&hash1,&prev_hash,&tran_list);
                                println!("----------------------------");
                                println!("{}",st);
                                println!("-----------------------------");
                                let mut file = match File::open(path){
                                    Err(why) => panic!("ERROR:  {}",why.description()),
                                    Ok(file) => file,
                                };
                                let mut st = format!("{}\n",st);
                                let mut file = OpenOptions::new()
                                                .write(true)
                                                .append(true)
                                                .open(path)
                                                .unwrap();
                                file.write_all(st.as_bytes());
                            tran_list.clear();
                        }
                }
                "5" => {
                        account.clear();
                        password.clear();
                        println!("You're going to log in");
                        println!("input account:    ");
                        io::stdin().read_line(&mut account).expect("ERROR: Invalid input");
                        println!("input password:   ");
                        io::stdin().read_line(&mut password).expect("ERROR: Invalid input");
                        let mut result = 0;
                        for i in 0..wallet_list.len(){
                            if wallet_list[i].account == account{
                                result = 1;
                                println!("Log in Successfull");
                                    'outer1: loop{
                                        input.clear();
                                        println!("-> 1- Making Transaction");
                                        println!("-> 2- View Blockchain");
                                        println!("-> 3- Log out");
                                        println!("<enter> your choice:  ");
                                        io::stdin().read_line(&mut input).expect("ERROR: Invalid input");
                                        match input.trim().as_ref(){
                                            "1" => {
                                                    println!("Making Transaction in progress");
                                                    let mut transaction = Transaction::transfer(&mut wallet_list);
                                                    tran_list.push(transaction);
                                                    println!("Do you want another process?");
                                                    io::stdin().read_line(&mut input1).expect("ERROR: Invalid input");
                                                        if input1.trim() != "y"{
                                                            break 'outer1;
                                                        }else{
                                                            input1.clear();
                                                        }
                                                    input1.clear();
                                            }
                                            "2" => println!("View Blockchain in progress"),
                                            "3" => {
                                                    // println!("Log out in progress");
                                                    break 'outer1;
                                            }
                                            _   => println!("Invalid input"),
                                        }
                                    }
                                }
                            }
                        if result == 0{
                            println!("SORRY Account not found");
                        }
                }
                "6" => {
                        println!("You're going to see unconfirm transaction");
                        block::output_person(&tran_list);
                }
                "7" => {
                        let mut st = String::new();
                        let mut file = match File::open(path){
                                    Err(why) => panic!("ERROR:  {}",why.description()),
                                    Ok(file) => file,
                        };
                        // let mut file = OpenOptions::new()
                        //                .write(true)
                        //                .append(true)
                        //                .open(path)
                        //                .unwrap();
                        let reader = BufReader::new(file);
                        for line in reader.lines(){
                            let line = line.unwrap();
                            // println!("----------");
                            // println!("{}",line);
                            // println!("----------");
                            let mut vec1 = vec![];
                            let mut vec2 = vec![];
                            let mut vec3 = vec![];
                            for s in line.split(|x| (x == ',') || (x == '[') || (x == ']')){
                                if vec1.len() < 4{
                                    vec1.push(s);
                                }else{
                                    vec2.push(s);
                                }
                            }
                            println!("------Block Detail------");
                            println!("\t\tBlock ID:             {}",vec1[0]);
                            println!("\t\tBlock Timestamp:      {}",vec1[1]);
                            println!("\t\tBlock Hash:           {}",vec1[2]);
                            println!("\t\tBlock Prev_Hash:      {}",vec1[3]);
                            // println!("vec1:     {:?}",vec1);
                            // println!("vec2:     {:?}",vec2);
                            // println!("vec2 length:  {}",vec2.len());
                            // for i in (0..vec2.len()).step_by(7){
                            //     println!("\t\t\t")
                            // }
                            for r in vec2.iter(){
                                if r != &""{
                                    vec3.push(r);
                                }
                            }
                            // println!("vec3:     {:?}",vec3);
                            // println!("vec3 len: {}",vec3.len());
                            if vec3.len() == 0{
                                println!("No Transaction Now");
                            }else{
                            let mut no: i32 = 1;
                                for i in (0..vec3.len()).step_by(7){
                                    println!("\t\t\t#Transaction {}#",no);
                                    println!("\t\t\t\tTran_addr:        {}",vec3[i]);
                                    println!("\t\t\t\tSender Public Key:    {}",vec3[i+1]);
                                    println!("\t\t\t\tReceiver Public Key:  {}",vec3[i+2]);
                                    println!("\t\t\t\tAmount:               {}",vec3[i+3]);
                                    println!("\t\t\t\tTimestamp:            {}",vec3[i+4]);
                                    println!("\t\t\t\tSender Asset:         {}",vec3[i+5]);
                                    println!("\t\t\t\tReceiver Asset:       {}",vec3[i+6]);
                                    no += 1;
                                }
                            }
                            println!("******************");
                        }
                }
                _   => println!("ERROR: invalid input"),
            }
    }

/*
    let mut bitcoin = String::from("bitcoin");
    let mut bitcoin_wallet_id = String::new();
    bitcoin_wallet_id = hash(&bitcoin);

    let mut vec1: Vec<Transaction> = Vec::new();
    let mut block1 = block::Block::genesis_block(&vec1);
    let mut blockchain: Vec<block::Block> = Vec::new();
    blockchain.push(block1);
    let mut account_list: Vec<wallet::Wallet> = Vec::new();
    let wallet1 = wallet::Wallet::create_wallet(&blockchain);
    let wallet2 = wallet::Wallet::create_wallet(&blockchain);

    account_list.push(wallet1);         //balance = 0.00
    account_list.push(wallet2);         //balance = 0.00
        wallet::output(&account_list);
    
    let mut vector: Vec<Transaction> = Vec::new();
    Transaction::mining(&mut account_list,bitcoin_wallet_id);
    wallet::output(&account_list);
    let mut second_tran = Transaction::transfer(&mut account_list);
    vector.push(second_tran);
    
    println!("******************************");
   
    for i in 0..vector.len(){
        vector[i].output();
    }
    println!("******************************");
    let mut search = String::new();
    println!("Searching for: ");
    io::stdin().read_line(&mut search).unwrap();
    let mut result = block::search_tran_detail(&vector,search);
    
    println!("######Result for searching");
    result.output();
    println!("#############################");
    
    // wallet::output(&account_list);
    for i in  account_list.iter(){
        i.show_wallet();
    }
*/
    // let mut a = block::search_tran_detail()
/*    // account_list.push(wallet3);
    // wallet::output(&account_list);
    println!("---------------------------------");
    let mut first_tran = Transaction::transfer(&mut account_list);
    // first_tran.output();
    // 1251601293114113138171164769613712917012936124238140631922362252472472211449821802321
    println!("---------------------------------");
    // wallet::output(&account_list);

    let mut second_tran = Transaction::transfer(&mut account_list);
    // second_tran.output();
    // wallet::output(&account_list);

    let mut vector: Vec<block::Transaction> = Vec::new();
    vector.push(first_tran);
    vector.push(second_tran);
        // for i in 0..vector.len(){
        //     vector[i].output();
        // }
    // let mut block1 = block::Block::genesis_block(&vector);
    let mut block1 = block::Block::genesis_block();
    // block::Block::show_chain(&vector);
    // block1.show_chain(&vector);
    block1.output_data();
    */

    // loop{
    //     let mut account_list: Vec<wallet::Wallet> = Vec::new();
    //     let mut input = String::new();
    //     io::stdin().read_line(&mut input).unwrap();
    //     let mut input: i32;
    //     // input = input1.trim().parse().expect("Invalid Convering");
    //     println!("<1>.  Create new Wallet");
    //     println!("<2>.  Make Transaction");
    //     println!("<3>.  Mining");
    //     match input[..]{
    //         "1"=> {    
    //                     let wallet = wallet::Wallet::create_wallet();
    //                     account_list.push(wallet);
    //                 },
    //     }
    // }

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++=

    // let mut account_list: Vec<Wallet> = Vec::new();

    // let wallet1 = create_wallet();
    // wallet1.show_wallet();

    // let wallet2 = create_wallet();
    // wallet2.show_wallet();

    // let wallet3 = create_wallet();
    // wallet3.show_wallet();


    

    // let mut st = String::from("Hello");
    // let mut result = v_1::hash(&st);
    // println!("{}",vec_to_str(&result));

    
    // fn_transaction::transfer(&account_list);
    // block::Transaction::transfer(&account_list);
    
    
    
    
    
    
    
    
    
    
    
    
    

    // println!("wallet1.wallet_id:    {:?}",wallet1.wallet_id);
    
    // let mut vec: [u8;32];
    // vec = wallet1.wallet_id;
    // println!("{:?}",vec);
    
    // let buf = &[0x41u8, 0x41u8, 0x42u8];
    // let s = String::from_utf8_lossy(buf);
    // println!("result: {}", s);

    // println!("{}",str::from_utf8(wallet1.wallet_id));
    // account_list.push(wallet1);
    // account_list.push(wallet2);
    // account_list.push(wallet3);


    // let mut t0 = block::Transaction::new();
    // t0.output();

    // let mut t1 = block::Transaction::new_instance(    
    //     vec![1;2],
    //     vec![2;2],
    //     100f64,
    //     // -100f64,
    //     // 100f64,
    // );
    // // t1.output();
    // let mut vector: Vec<Transaction> = Vec::new();
    // vector.push(t1);
    // let mut block1 = block::Block::genesis_block(&vector);
    // block1.output_data();
    // let mut t2 = block::Transaction::new_instance(    
    //     vec![2;2],
    //     vec![3;2],
    //     9.9f64,
    // );
    // // t2.output();

    // let mut tran_data: Vec<Transaction> = Vec::new();
    // tran_data.push(t1);
    // tran_data.push(t2);
    //     for value in &tran_data{
    //         value.output();
    //     }


    // let mut person1 = block::Person::new();
    // // person1.output();
    // let mut person2 = block::Person{
    //     public_key: "1_public_key".to_string(),
    //     private_key: "1_private_key".to_string(),
    //     balance:    1000u64,
    // };
    // // person2.output();
    // let mut person3 = block::Person{
    //     public_key: "2_public_key".to_string(),
    //     private_key: "2_private_key".to_string(),
    //     balance:    900u64,
    // };
    // // person3.output();

    // let mut vector_person: Vec<block::Person> = Vec::new();
    // vector_person.push(person1);
    // vector_person.push(person2);
    // vector_person.push(person3);
    
    // let mut first_block = Block::genesis_block(&vector_person);
    // first_block.output_data();

    // let mut person4 = block::Person{
    //     public_key: "3_public_key".to_string(),
    //     private_key: "3_private_key".to_string(),
    //     balance:    800u64,
    // };
    // vector_person.push(person4);
    // for value in &vector_person{
    //     value.output();
    // }

}
