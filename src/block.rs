// extern crate chrono;
// use chrono::{DateTime,Utc};
// pub type Hash = Vec<u8>;
// pub type Human = Vec<Person>;

// struct block__chain{
//     block_data: Vec<Block>,
// }

// pub struct Person{
//     public_key:     String,
//     private_key:    String,
//     balance:        u64,
// }
// impl Person{
//     pub fn new() -> Self{
//         Person{ 
//             public_key:     "none_public_key".to_string(),
//             private_key:    "none_private_key".to_string(),
//             balance:        0u64,
//         }
//     }
//     pub fn new_instance(public_key: String,private_key: String,balance: u64) -> Person{
//         Person{
//             public_key,
//             private_key,
//             balance,
//         }
//     }
//     pub fn output(&self){
//         println!("\tPublic_Key:   {}",format!("*****{}*****",&self.public_key));
//         println!("\tPrivate_Key:   {}",format!("*****{}*****",&self.private_key));
//         println!("\tBalance:      {}",format!("#####${}#####",&self.balance));
//         println!("\t------------------------------\n");
//     }
// }



// impl block__chain{
//     pub fn new() ->  Self{
//         block__chain{
//             block_data: vec![],
//         }
//     }
//     pub fn push_block(&mut self,new_block: Block){
//         &self.block_data.push(new_block);
//     }
//     pub fn show_block(&self){
//         for value in &self.block_data{
//            Block::output_block(value); 
//         }
//     }
// }
//     use std::io;
//     pub trait BlockChain{
//         fn genesis_block(human: Human) -> Block;
//         fn block_instance(&self,human: Human) -> Block;
//         fn mine_block(&self,block: &[Block]);
//         fn input_data(&mut self);
//         fn output_data(&self);
//         fn output_block(block: &Block);
//         fn show_chain(&self,blockchain: &[Block]);
//         fn combine_string(&self) -> String;
//         fn checking_hash(blocks: &Vec<Block>) -> bool;
//     }
//     pub struct Block{
//         id:         u32,
//         timestamp:  String,
//         hash: Hash,
//         pub prev_hash:  Hash,
//         data:       Human,
//     }

//     impl BlockChain for Block{
//         fn genesis_block(human: Human) -> Block{
//             let time = current_time();
//             let mut value_hash: Vec<u8> = Vec::new();
//             let mut st1 = human_to_str(&human);
//             st1 = st1 + &time;
//             value_hash = hash(&st1);
//             Block{
//                 id: 0u32,
//                 timestamp: time,
//                 hash: value_hash, 
//                 prev_hash: vec![0;32],
//                 data: human,
//             }
//         }


//         fn block_instance(&self,human: Human) -> Block{
            
//             let time = current_time();
//             let mut value_hash: Vec<u8> = Vec::new();
//             let mut st1 = human_to_str(&human);
//             st1 = st1 + &time;
//             value_hash = hash(&st1);
//             Block{
//                 id: &self.id + 1,
//                 timestamp: time,
//                 hash: value_hash,
//                 prev_hash: self.hash.clone(),
//                 data: human,
//             }
//         }

//         fn mine_block(&self, block: &[Block]){
//             let mut data = String::new();
//             io::stdin().read_line(&mut data).expect("Invalid input");
//         }

//         fn input_data(&mut self){
//             let mut id = String::new();
//             println!("input id: ");
//             io::stdin().read_line(&mut id).expect("Something went wrong!");
//             self.id = id.trim().parse().unwrap();
//         }
//         fn output_data(&self){
//             println!("ID:           {}",&self.id);
//             println!("Timestamp:    {}",&self.timestamp);
//             println!("Prev_hash:    {}",vec_to_str(&self.prev_hash));
//             println!("Hash:         {}",vec_to_str(&self.hash));
//             println!("Data: ->");
//             output_person(&self.data);
//         }
//         fn output_block(block: &Block){
//             println!("ID:           {}",block.id);
//             println!("Timestamp:    {}",block.timestamp);
//             println!("Prev_hash:    {}",vec_to_str(&block.prev_hash));
//             println!("Hash:         {}",vec_to_str(&block.hash));
//             println!("Data: ->");
//             output_person(&block.data);
//         }
//         fn show_chain(&self,blockchain: &[Block]){
//             for block in blockchain{
//                 Block::output_block(&block);
//                 println!("----------*----------");
//             }
//         }

//         fn combine_string(&self) -> String{
//             let mut st = String::new();
//             st = st + &self.id.to_string() + &self.timestamp + &vec_to_str(&self.hash) + &vec_to_str(&self.prev_hash) /*+ vec_to_str1(&self.data)*/;
//             st
//         }
//         fn checking_hash(blocks: &Vec<Block>) -> bool{
//             true
//         }
        
//     }

// pub fn current_time() -> String{
//         let now: DateTime<Utc> = Utc::now();
//         let mut time = String::new();
//         time = now.format("%a %e %b %Y %T.%f").to_string();
//         time
// }

// pub fn hash (data: &String) -> Hash {
//             crypto_hash::digest(crypto_hash::Algorithm::SHA256, data.as_bytes())
//         }

// pub fn vec_to_str(vector: &Vec<u8>) -> String{
//     let mut st = String::new();
//     st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
//     st
// }

// pub fn human_to_str(human:  &Human) -> String{
//     let mut totalString = format!("");
//     let mut totalString1 = format!("");
// pub fn human_to_str(human:  &Human) -> String{
//     let mut totalString = format!("");
//     let mut totalString1 = format!("");
//     for value in human{
//         totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
//         totalString = totalString + &totalString1;
//     }
//     return totalString
// }
//     for value in human{
//         totalString1 = format!("{}{}{}",value.public_key,value.private_key,value.balance);
//         totalString = totalString + &totalString1;
//     }
//     return totalString
// }

// pub fn output_person(vector: &Vec<Person>){
//     for value in vector.iter(){
//         value.output();
//     }
// }

// fn st(s: &String) -> String{
//     s.to_string()
// }

// fn main(){
    
// }
///1111111111
// extern crate chrono;
// extern crate chrono;
// extern crate v_1;

// use v_1::current_time;
// use v_1::human_to_str;
// use v_1::output_person;
// use v_1::st;

// use v_1::Wallet;
// use v_1::{self,account};
// use wallet::Wallet;
// pub type Hash = Vec<u8>;
// mod lib;
// extern crate v_1;


use v_1::hash;
use v_1::vec_to_str;
use v_1::Hash;
// use v_1::Wallet;
// mod wallet;
// use wallet::Wallet;
use crate::wallet::Wallet;
use crate::wallet::Valid;

use std::collections::HashMap;
use std::collections::HashSet;
// use crate::wallet;
// use crate::wallet::show_valid;
use chrono::{DateTime,Utc};
pub type Transact = Vec<Transaction>;



pub struct Transaction{
    pub tran_addr:              String,
    pub sender_public_key:      String,
    pub receiver_public_key:    String,
    // pub sender_signature:       Hash,
    pub amount:                 f64,
    pub timestamp:              String,
    // pub isvalid:                bool,
    pub sender_asset:           f64,
    pub receiver_asset:         f64,
}
pub trait fn_transaction{
    fn new() -> Self;
    fn new_instance(tran_addr: String,sender_public_key: String,receiver_public_key: String,/*sender_signature: Hash,*/amount: f64) -> Transaction;
    fn output(&self);
    fn transfer(vec: &mut Vec<Wallet>) -> Transaction;
    fn mining(vec: &mut Vec<Wallet>, bit:  String) -> Transaction;
}
impl fn_transaction for Transaction{
    fn new() -> Self{
        Transaction{
            tran_addr:              vec_to_str(&vec![0;1]),
            sender_public_key:      vec_to_str(&vec![0;1]),
            receiver_public_key:    vec_to_str(&vec![0;1]),
            amount:                 0.00,
            timestamp:              "null".to_string(),
            sender_asset:           0.00,
            receiver_asset:         0.00,
        }
    }
    fn new_instance(tran_addr: String,sender_public_key: String,receiver_public_key: String,/*sender_signature: Hash,*/amount: f64) -> Transaction{
        
        Transaction{
              tran_addr,
              sender_public_key,
              receiver_public_key,
            //   sender_signature:
              amount,
              timestamp:    current_time(),
              sender_asset:     -amount,
              receiver_asset:    amount,
            //   isvalid:
        }
    }
    // fn transfer(vec: &mut Vec<Wallet>) -> Transaction{
    //     let mut a = String::new();
    //     let mut b = String::new();
    //     let mut c = String::new();
    //     let mut d = String::new();
    //     let mut e = String::new();
    //     let mut f = String::new();
    //     let mut d2: f64;
    //     let mut e2: f64;
    //     let mut f2: f64;
    //     let mut x = String::new();
    //     let mut sender_pk = String::new();
    //     let mut sender_pw = String::new();
    //     let mut sender_balance: f64 = 0.00;
    //     let mut receiver_pk = String::new();
    //     let mut amount1 = String::new();
    //     let mut amount2: f64;
    //     let mut amount3: f64 = 0.00;
    //     let mut t = 0;
    //     let mut sender_wallet_id = String::new();
    //     let mut receiver_wallet_id = String::new();
    //     let time = current_time();
    //     println!("enter Sender PK:  ");
    //     io::stdin().read_line(&mut sender_pk).expect("Invalid input"); 
    //     'outer: for i in 0..vec.len(){
    //         if vec[i].account == sender_pk{
    //             println!("enter Sender Password:    ");
    //             io::stdin().read_line(&mut sender_pw).expect("Invalid input");
    //                 if vec[i].password == sender_pw{
    //                     sender_wallet_id = vec[i].wallet_id.clone();
    //                             for a in 0..vec[i].valid_addr.len(){
    //                                 sender_balance += vec[i].valid_addr[a].amount;
    //                             }
    //                     println!("enter Receiver PK:    ");
    //                     io::stdin().read_line(&mut receiver_pk).expect("Invalid input");  
    //                         for i in 0..vec.len(){
    //                             if vec[i].account == receiver_pk{
    //                                 receiver_wallet_id = vec[i].wallet_id.clone();
    //                                 println!("Now you can make transaction to-> {}",receiver_pk);
    //                                 println!("enter Amount for Transfer:    ");
    //                                 io::stdin().read_line(&mut amount1).expect("Invalid input");
    //                                 amount2 = amount1.trim().parse().expect("Converting Error");
    //                                     if sender_balance <= amount2{
    //                                         println!("ERROR:  Lack of Balance");
    //                                         let mut a1 = String::from("null");
    //                                         let mut b1 = String::from("null");
    //                                         let mut c1 = String::from("null");
    //                                         let mut d1 = String::from("0.00");
    //                                         let mut e1 = String::from("0.00");
    //                                         let mut f1 = String::from("0.00");
    //                                         a = a1.clone();
    //                                         b = b1.clone();
    //                                         c = c1.clone();
    //                                         d = d1.clone();
    //                                         e = e1.clone();
    //                                         f = f1.clone();
    //                                         break 'outer;
    //                                     }else{
    //                                         // let mut string = format!("{}{}{}{}{}{}",
    //                                         //     sender_wallet_id,
    //                                         //     receiver_wallet_id,
    //                                         //     amount1,
    //                                         //     time,
    //                                         //     amount1,
    //                                         //     amount1,
    //                                         // );
    //                                         // a = hash(&string);
    //                                         // b = sender_wallet_id.clone();
    //                                         // c = receiver_wallet_id.clone();
    //                                         // d = amount1.clone();
    //                                         // e = amount1.clone();
    //                                         // f = amount1.clone();
    //                                         //     let mut vec_wallet = Wallet::new();
    //                                         //     let mut temp: Vec<Valid> = Vec::new();
    //                                         //     let mut vec_wallet = &vec[t];
    //                                         //     amount3 = amount1.trim().parse().expect("ERROR: Converting Error");
    //                                         //         for a in 0..vec_wallet.valid_addr.len(){
    //                                         //             // sender_balance += vec_wallet.valid_addr[a].amount;
    //                                         //             // if sender_balance < amount3{
    //                                         //             //     temp.push(vec_wallet.valid_addr[a]);
    //                                         //             // }
    //                                         //             // vec_wallet.valid_addr[a].show();
    //                                         //         }
    //                                         let mut x1 = String::from("1");
    //                                         x = x1.clone();
    //                                         break 'outer;    
    //                                     }
    //                                 }
    //                             }  
    //                         }
    //                 }
    //         t += 1;
    //     }
    //     // Transaction{
    //     //     tran_addr:              "null".to_string(),
    //     //     sender_public_key:      "null".to_string(),
    //     //     receiver_public_key:    "null".to_string(),
    //     //     amount:                 0.00,
    //     //     timestamp:              time,
    //     //     sender_asset:           0.00,
    //     //     receiver_asset:         0.00,
    //     // }
    //         if x == "1".to_string(){
    //             let mut string = format!("{}{}{}{}{}{}",
    //                 sender_wallet_id,
    //                 receiver_wallet_id,
    //                 amount1,
    //                 time,
    //                 amount1,
    //                 amount1,
    //             );
    //                                         a = hash(&string);
    //                                         b = sender_wallet_id.clone();
    //                                         c = receiver_wallet_id.clone();
    //                                         d = amount1.clone();
    //                                         e = amount1.clone();
    //                                         f = amount1.clone();
    //                                         let mut i = 0;
    //                                         let mut i1 = 0;
    //                                         let mut i2 = 0;
    //                                             let mut vec_wallet = Wallet::new();
    //                                             let mut temp: HashMap<i32,String> = HashMap::new();
    //                                             let mut temp1: HashMap<i32,String> = HashMap::new();
    //                                             // let mut temp: Vec<&Valid> = Vec::new();
    //                                             // let mut temp1: Vec<&Valid> = Vec::new();
    //                                             let mut vec_wallet = &mut vec[t];
    //                                             amount3 = amount1.trim().parse().expect("ERROR: Converting Error");
    //                                         //         for a in 0..vec_wallet.valid_addr.len(){
    //                                         //             sender_balance += vec_wallet.valid_addr[a].amount;
    //                                         //             if sender_balance < amount3{
    //                                         // //             //     temp.push(vec_wallet.valid_addr[a]);
    //                                         //                 println!("{}",sender_balance);
    //                                         //             }
    //                                         // //             // vec_wallet.valid_addr[a].show();
    //                                         //         }
    //                                             for res in vec_wallet.valid_addr.iter(){
    //                                                 sender_balance += res.amount;
    //                                                     if sender_balance < amount3{
    //                                                         // temp.push(res);
    //                                                         temp.insert(i as i32,res.address.clone());
    //                                                     }else if sender_balance == amount3{
                                                            
                                                            
                                                            
    //                                                         for (integer,string) in temp.iter(){
    //                                                             // let mut i = 0;
    //                                                             for res2 in vec_wallet.valid_addr.iter(){
    //                                                                 if *string != res.address{
    //                                                                     // vec_wallet.valid_addr.remove(i);
    //                                                                     temp1.insert(0i32,res.address.clone());
    //                                                                 }
    //                                                             }
    //                                                         }
    //                                                     // let mut ve: Vec<Valid> = temp1.iter().map;
    //                                                         println!("-------------");
    //                                                             for (key,value) in temp1.iter(){
    //                                                                 println!("key:      {}",key);
    //                                                                 println!("value:    {}",value);
    //                                                             }
    //                                                         println!("-------------");


    //                                                     }else{
    //                                                         //making change
    //                                                     }
    //                                             }
    //         }else{
    //             println!("No value")
    //         }
    //     println!("T value = {}",t);
    //     d2 = d.trim().parse().unwrap();
    //     e2 = e.trim().parse().unwrap();
    //     f2 = f.trim().parse().unwrap();
    //     Transaction{
    //         tran_addr:              a,
    //         sender_public_key:      b,
    //         receiver_public_key:    c,
    //         amount:                 d2,
    //         timestamp:              time,
    //         sender_asset:           e2,
    //         receiver_asset:         f2,
    //     }
        
    // }
    fn transfer(vec: &mut Vec<Wallet>) -> Transaction{
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();
        let mut d = String::new();
        let mut e = String::new();
        let mut f = String::new();
        let mut d2: f64;
        let mut e2: f64;
        let mut f2: f64;
        // let mut x = String::new();
        let mut x = 0;
        let mut sender_pk = String::new();
        let mut sender_pw = String::new();
        let mut sender_balance: f64 = 0.00;
        let mut receiver_pk = String::new();
        let mut amount1 = String::new();
        let mut amount2: f64;
        let mut amount3: f64 = 0.00;
        let mut t = 0;
        let mut sender_wallet_id = String::new();
        let mut receiver_wallet_id = String::new();
        let time = current_time();
        println!("enter Sender PK:  ");
        io::stdin().read_line(&mut sender_pk).expect("Invalid input"); 
        'outer: for i in 0..vec.len(){
            if vec[i].account == sender_pk{
                println!("enter Sender Password:    ");
                io::stdin().read_line(&mut sender_pw).expect("Invalid input");
                sender_wallet_id = vec[i].wallet_id.clone();
                sender_balance = vec[i].balance;
                    if vec[i].password == sender_pw{
                        println!("enter Receiver PK:    ");
                        io::stdin().read_line(&mut receiver_pk).expect("Invalid input");  
                            for i in 0..vec.len(){
                                if vec[i].account == receiver_pk{
                                    receiver_wallet_id = vec[i].wallet_id.clone();
                                    println!("Sender Balance:   {}",sender_balance);
                                    println!("Now you can make transaction to-> {}",receiver_pk);
                                    println!("enter Amount for Transfer:    ");
                                    io::stdin().read_line(&mut amount1).expect("Invalid input");
                                    amount2 = amount1.trim().parse().expect("Converting Error");
                                        if sender_balance <= amount2{
                                            println!("ERROR:  Lack of Balance");
                                            let mut a1 = String::from("null");
                                            let mut b1 = String::from("null");
                                            let mut c1 = String::from("null");
                                            let mut d1 = String::from("0.00");
                                            let mut e1 = String::from("0.00");
                                            let mut f1 = String::from("0.00");
                                            a = a1.clone();
                                            b = b1.clone();
                                            c = c1.clone();
                                            d = d1.clone();
                                            e = e1.clone();
                                            f = f1.clone();
                                            break 'outer;
                                        }else{
                                            println!("Enough Balance");
                                            // vec[i].balance += amount2;
                                            x = 1;
                                            break 'outer;    
                                        }
                                    }
                                }  
                            }
                    }
            t += 1;
        }
        println!("x {}",x);    
        // if x == "1".to_string(){
        amount2 = amount1.trim().parse().expect("ERROR:  Converting Error");
        // amount2 = amount1.trim().parse().expect("Converting Error");
        println!("Sender:   {}",sender_pk);
        println!("Receiver: {}",receiver_pk);
        if x == 1{
            let mut string = format!("{}{}{}{}{}{}",
                sender_wallet_id,
                receiver_wallet_id,
                amount1,
                time,
                amount1,
                amount1,
            );
                for r in 0..vec.len(){
                    if vec[r].account == sender_pk{
                        // println!("You found sender_wallet_id");
                        // vec[r].balance -= amount2 + 0.1;
                        println!("NOTE: Transaction in processing, check your transaction in next 10mnn at least");
                        // println!("Balance:  {}",vec[r].balance);
                    }else if vec[r].account == receiver_pk{
                        // println!("You found receiver_wallet_id");
                        // vec[r].balance += amount2;
                        println!("NOTE: Transaction in processing, check your transaction in next 10mnn at least");
                        // println!("Balance:  {}",vec[r].balance);
                    }
                }
            println!("Result to put inside return Transaction");
            println!("Transaction Addresss:     {}",hash(&string));
            println!("Sender Public Key:        {}",sender_wallet_id);
            println!("Receiver Public Key:      {}",receiver_wallet_id);
            println!("Sending amount:           {}",amount2);
            return Transaction{
                tran_addr:              hash(&string),
                sender_public_key:      sender_wallet_id,
                receiver_public_key:    receiver_wallet_id,
                amount:                 amount2,
                timestamp:              time,
                sender_asset:           -amount2,
                receiver_asset:         amount2,
            }
        }else{
                // println!("No value")
            return Transaction{
                tran_addr:              "null".to_string(),
                sender_public_key:      "null".to_string(),
                receiver_public_key:    "null".to_string(),
                amount:                 0.00,
                timestamp:              time,
                sender_asset:           0.00,
                receiver_asset:         0.00,
            }
        }
    }
        
    fn mining(vec: &mut Vec<Wallet>,bit: String) -> Transaction {
        let mut amount = String::new();
        let mut amount1: f64;
        let mut receiver = String::new();
        let mut pk = String::new();
        let mut a: i32 = 0;
        let mut data = String::new();
        let mut time = String:: new();
        time = current_time();
        println!("Send to: ");
        io::stdin().read_line(&mut receiver).expect("Invalid input");
            for i in 0..vec.len(){
                if vec[i].account == receiver{
                    a = 1;
                    pk = vec[i].wallet_id.clone();
                    println!("Amount:   ");
                    io::stdin().read_line(&mut amount).expect("Invalid input");
                    amount1 = amount.trim().parse().expect("Converting Error");
                    vec[i].balance = amount1;
                    data = format!("{}{}{}{}{}{}",
                            bit,
                            receiver,
                            amount1.to_string(),
                            time,
                            amount1.to_string(),
                            amount1.to_string(),    
                    );
                    // let mut valid = Valid::new(hash(&data),amount1);
                    // vec[i].valid_addr.push(valid);
                }
            }
        amount1 =  amount.trim().parse().expect("ERROR: Converting Erro");
        println!("Data: {}",data);
        Transaction{
            tran_addr:              hash(&data),
            sender_public_key:      bit,
            receiver_public_key:    pk,
            amount:                 amount1,
            timestamp:              time,
            sender_asset:           -amount1,
            receiver_asset:         amount1,   
        }

    }
    fn output(&self){
        // println!("\tPublic_Key:   {}",format!("*****{}*****",&self.public_key));
        // println!("\tPrivate_Key:   {}",format!("*****{}*****",&self.private_key));
        // println!("\tBalance:      {}",format!("#####${}#####",&self.balance));
        println!("----------Transaction On {}----------",&self.timestamp);
        println!("Transaction Address:  {}",&self.tran_addr);
        println!("Sender Public Key:    {}",format!("{}_key",&self.sender_public_key));
        println!("Receiver Public Key:  {}",format!("{}_key",&self.receiver_public_key));
        println!("Transfer Amount:      $ {}",&self.amount);
        println!("Sender:               $ {}",&self.sender_asset);
        println!("Receiver:             $ {}",&self.receiver_asset);
        println!("\t------------------------------\n");
    }
}

pub struct block__chain<'a,'b>{
    pub block_data: &'b mut Vec<Block<'a>>,
}

impl<'a,'b> block__chain<'a,'b>{
    pub fn push_block(&mut self,new_block: Block<'a>){
        &self.block_data.push(new_block);
    }
    pub fn show_block(&self){
        for value in self.block_data.iter(){
           Block::output_block(value); 
        }
    }
}
    use std::io;
    pub trait BlockChain<'a>{
        fn genesis_block(transaction: &Transact) -> Block;
        fn block_instance(&self,transaction: &'a Transact) -> Block;
        fn mine_block(&self,block: &[Block]);
        fn input_data(&mut self);
        fn output_data(&self);
        fn output_block(block: &Block);
        fn show_chain(&self,blockchain: &[Block]);
        fn combine_string(&self) -> String;
        fn checking_hash(blocks: &Vec<Block>) -> bool;
        // fn mining(wall: &wallet::Wallet,bit: String) -> Transaction;
    }
    pub struct Block<'a>{
        pub id:         u32,
        pub timestamp:  String,
        pub hash:       String,
        pub prev_hash:  String,
        pub data:       &'a Transact,
    }

    impl<'a> BlockChain<'a> for Block<'a>{
        // fn new() -> Self{
        //     Block{
        //         id: 0u32,
        //         timestamp: "null".to_string(),
        //         hash:      "null".to_string(),
        //         prev_hash:  "null".to_string(),
        //         data:   
        //     }
        // }
        fn genesis_block(transaction: &Transact) -> Block{
            let time = current_time();
            let mut value_hash = String::new();
            // let mut value_hash: Vec<u8> = Vec::new();
            let mut st1 = human_to_str(&transaction);
            st1 = st1 + &time;
            value_hash = hash(&st1);
            Block{
                id: 0u32,
                timestamp: time,
                hash: value_hash, 
                prev_hash: vec_to_str(&vec![0;32]),
                data: transaction,
            }
        }


        fn block_instance(&self,transaction: &'a Transact) -> Block{
            
            let time = current_time();
            let mut value_hash = String::new();
            // let mut value_hash: Vec<u8> = Vec::new();
            let mut st1 = human_to_str(&transaction);
            st1 = st1 + &time;
            value_hash = hash(&st1);
            Block{
                id: &self.id + 1,
                timestamp: time,
                hash: value_hash,
                prev_hash: self.hash.clone(),
                data: transaction,
            }
        }

        fn mine_block(&self, block: &[Block]){
            let mut data = String::new();
            io::stdin().read_line(&mut data).expect("Invalid input");
        }

        fn input_data(&mut self){
            let mut id = String::new();
            println!("input id: ");
            io::stdin().read_line(&mut id).expect("Something went wrong!");
            self.id = id.trim().parse().unwrap();
        }
        fn output_data(&self){
            println!("ID:           {}",&self.id);
            println!("Timestamp:    {}",&self.timestamp);
            println!("Prev_hash:    {}",&self.prev_hash);
            println!("Hash:         {}",&self.hash);
            println!("Data: ->");
            output_person(&self.data);
        }
        fn output_block(block: &Block){
            println!("ID:           {}",block.id);
            println!("Timestamp:    {}",block.timestamp);
            println!("Prev_hash:    {}",&block.prev_hash);
            println!("Hash:         {}",&block.hash);
            println!("Data: ->");
            output_person(&block.data);
        }
        fn show_chain(&self,blockchain: &[Block]){
            for block in blockchain{
                Block::output_block(&block);
                println!("----------*----------");
            }
        }

        fn combine_string(&self) -> String{
            let mut st = String::new();
            st = st + &self.id.to_string() + &self.timestamp + &self.hash + &self.prev_hash /*+ vec_to_str1(&self.data)*/;
            st
        }
        fn checking_hash(blocks: &Vec<Block>) -> bool{
            true
        }
    //     pub sender_public_key:      String,
    //     pub receiver_public_key:    String,
    //     // pub sender_signature:       Hash,
    //     pub amount:                 f64,
    //  pub timestamp:              String,
    // // pub isvalid:                bool,
    // pub sender_asset:           f64,
    // pub receiver_asset:         f64,
        // fn mining(wall: &wallet::Wallet,bit: String) -> Transaction{
            
        //     Transaction{
        //         sender_public_key:      bit,
        //         receiver_public_key:    
        //         amount: 
        //         timestamp:
        //         sender_asset:
        //         receiver_aset:
        //     }
        // }
        
    }

pub fn current_time() -> String{
        let now: DateTime<Utc> = Utc::now();
        let mut time = String::new();
        time = now.format("%a %e %b %Y %T.%f").to_string();
        time
}

// pub fn more_detail(vector: &mut Vec<Transaction>,st: String) -> Transaction{
//     for i in 0..vector.len(){
//         if 
//     }
// }
pub fn search_tran_detail(vector: &Vec<Transaction>,st: String) -> Transaction{
    let mut a1 = 0;
    // let mut a1 = 0;
    // let mut vec: Vec<Transaction> = Vec::new();
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d: f64;
    let mut f = String::new();
    let mut g: f64;
    let mut h: f64;
    for i in 0..vector.len(){
        if vector[i].tran_addr == st.trim(){
            println!("FOUNDING................................");
            a1 = 1;
            // let mut vec = vector[i];
            // break;
            // return Transaction{
            //     tran_addr:              vector[i].tran_addr,
            //     sender_public_key:      vector[i].sender_public_key,
            //     receiver_public_key:    vector[i].receiver_public_key,
            //     amount:                 vector[i].amount,
            //     timestamp:              vector[i].timestamp,
            //     sender_asset:           vector[i].sender_asset,
            //     receiver_asset:         vector[i].receiver_asset,   
            // }
            a = vector[i].tran_addr.clone();
            b = vector[i].sender_public_key.clone();
            c = vector[i].receiver_public_key.clone();
            d = vector[i].amount;
            f = vector[i].timestamp.clone();
            g = vector[i].sender_asset;
            h = vector[i].receiver_asset;
            return Transaction{
                tran_addr:              a,
                sender_public_key:      b,
                receiver_public_key:    c,
                amount:                 d,
                timestamp:              f,
                sender_asset:           g,
                receiver_asset:         h,   
            }
        }
        // a1 += 1;
    }
    // if a == 1{
    //     return vector1[a1];
    // }else{ receiver: String,
        // if a1 == 1{
        //     Transaction{
        //         tran_addr:              a,
        //         sender_public_key:      b,
        //         receiver_public_key:    c,
        //         amount:                 d,
        //         timestamp:              f,
        //         sender_asset:           g,
        //         receiver_asset:         h,   
        //     }
    // }else{
        Transaction{
            tran_addr:              "null".to_string(),
            sender_public_key:      "null".to_string(),
            receiver_public_key:    "null".to_string(),
            amount:                 0.00,
            timestamp:              "null".to_string(),
            sender_asset:           0.00,
            receiver_asset:         0.00,   
        }
    // }
}

fn main(){

}
// static vector: Vec<Block> = vec![];
pub fn return_vec_block<'a,'b>(vec: &'a Vec<Block>) -> &'b Vec<Block<'a>>{
    // let mut blockchain: Vec<Block> = Vec::new();
    // &blockchain
    &vec
}
pub fn human_to_str(transaction:  &Transact) -> String{
    let mut totalString = format!("");
    let mut totalString1 = format!("");
    for value in transaction{
        totalString1 = format!("{}{}{}{}",&value.sender_public_key,&value.sender_public_key,value.amount,value.timestamp);
        totalString = totalString + &totalString1;
    }
    return totalString
}

pub fn output_person(vector: &Vec<Transaction>){
    for value in vector.iter(){
        value.output();
    }
}

pub fn verify_transaction(vec: &mut Vec<Wallet>,sender: String, balance: f64) -> bool{
    for i in 0..vec.len(){
        if vec[i].account == sender{
            if vec[i].balance < balance{
                return false
            }
        }
    }
    true
}
pub fn combine(s1: &String, s2: &String, s3: &String, s4: &String, vec: &Vec<Transaction>) -> String{
    let mut st = String::new();
    let mut sign1 = String::from("[");
    let mut sign2 = String::from("]");
    let mut split_sign = String::from(",");
        st = format!("{},{},{},{},",s1.trim(),
                                    s2.trim(),
                                    s3.trim(),
                                    s4.trim());
        if vec.len() != 1{
            for result in vec.iter(){
                st = st + &format!("[{},{},{},{},{},{},{}]",result.tran_addr.trim(),
                                                              result.sender_public_key.trim(),
                                                              result.receiver_public_key.trim(),
                                                              result.amount.to_string(),
                                                              result.timestamp.trim(),
                                                              result.sender_asset.to_string(),
                                                              result.receiver_asset.to_string());
            }
        }else{
            for result in vec.iter(){
                st = st + &format!("[{},{},{},{},{},{},{}]\n",result.tran_addr.trim(),
                                                              result.sender_public_key.trim(),
                                                              result.receiver_public_key.trim(),
                                                              result.amount.to_string(),
                                                              result.timestamp.trim(),
                                                              result.sender_asset.to_string(),
                                                              result.receiver_asset.to_string());    
            }
        }
            
    st
}

// return Transaction{
//                 tran_addr:              a,
//                 sender_public_key:      b,
//                 receiver_public_key:    c,
//                 amount:                 d,
//                 timestamp:              f,
//                 sender_asset:           g,
//                 receiver_asset:         h,   
//             }
pub fn struct_to_str(vec: &Vec<Transaction>) -> String{
    let mut st = String::new();
        for value in vec.iter(){
            st = st + &format!("{}{}{}{}{}{}{}",
                value.tran_addr,
                value.sender_public_key,
                value.receiver_public_key,
                value.amount.to_string(),
                value.timestamp,
                value.sender_asset.to_string(),
                value.receiver_asset.to_string(),
            );
        }
    st
}

