struct block__chain<'a,'b>{
    block_data: &'b Vec<Block<'a>>,
}
impl<'a,'b> block__chain<'a,'b>{
    fn output(&self){
        for value in self.block_data.iter(){
            value.output();
        }
    }
}
struct Block<'a>{
    data: &'a Vec<Person>,
}
impl<'a> Block<'a>{
    fn output(&self){
        for value in self.data.iter(){
            value.output();
        }
    }
}

use std::cmp::PartialEq;
// #[derive(Hash, Eq, PartialEq, Debug)]
struct Person{
    id:     u32,
    name:   String,
}
impl Person{
    fn output(&self){
        println!("id:   {}",&self.id);
        println!("name: {}",&self.name);
        println!("----------");
    }
}
// static  mut public_value: i32 = 10;
use std::num::ParseIntError;
use std::io;
// extern crate v_1;
// use v_1::hash;
use std::str;
pub fn vec_to_str(vector: &Vec<u8>) -> String{
    let mut st = String::new();
    st = vector.into_iter().map(|i| i.to_string()).collect::<String>();
    st
}
// use bitcoin::blockdata::script::Script;
// fn return_value() -> i32{
//     let mut a = 0;
//     let mut b = 0;
//     let mut c = 0;
//     let mut d = 0;
//     let mut value: i32;
//     let mut decide: i32;
//     for i in 0..10{
        
//     }
//         if decide == 0{
//             // println!("Incorrect");
//             return 0;
//         }else{
//             // println!("Correct");
//             return 1;
//         }
// }
use std::collections::HashMap;
use std::collections::HashSet;

// struct User {
//     reference: String,
//     email: String
// }

fn remove_hashmap(x: &mut HashMap<i32, String>,remove: i32) {
    let tmp = x.clone();
    let empties = tmp
         .iter()
         .filter(|&(&v, _)| v == remove)
         .map(|(k, _)| k);

    for k in empties { x.remove(k); }
}
// pub struct Person{
//     id:     i32,
//     name:   String,
// }
struct B{
    value: Vec<i32>,
}
// fn convert(vec:&mut Vec<&Person>) -> Vec<Person>{
//     let mut vector: Vec<Person> = Vec::new();
//     for r in 0..vec.len(){
//         vector.push(*vec[r]);
//     }
//     // for r in vec.iter(){
//     //     vector.push(**r);
//     // }
//     vector
// }
fn change(vec: &mut Vec<Person>){
    let mut id = String::new();
    let mut id1: u32;
    println!("search for id:    ");
    io::stdin().read_line(&mut id).unwrap();
    id1 = id.trim().parse().unwrap();
    for i in 0..vec.len(){
        if vec[i].id == id1{
            vec[i].id += 5;
        }
    }
}
fn main(){
    // let mut vector: Vec<i32> = Vec::new();
    // vector.push(1);
    // vector.push(2);
    // vector.push(3);
    // for i in vector.iter(){
    //     println!("{}",i);
    // }
    // change(&mut vector);
    // for i in vector.iter(){
    //     println!("{}",i);
    // }

    // let mut vec: Vec<i32> = Vec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // let mut object = B{
    //         value:  vec,
    // };
    // for i in 0..object.value.len(){

    // }
    // println!("{:?}",object);
//     let mut temp = HashSet::new();
//     temp.insert(Person{id: 1u32, name: "a".to_string()});
//     temp.insert(Person{id: 2u32, name: "b".to_string()});
//     temp.insert(Person{id: 3u32, name: "c".to_string()});

//         for i in &temp{
//             println!("{:?}",i);
//         }

    // let mut vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i32> = Vec::new();
    // let mut vec1: HashSet<i32> = HashSet::new();
    // let mut vec2: HashSet<i32> = HashSet::new();
    // vec1.insert(1);
    // vec1.insert(2);
    // vec1.insert(3);

    // vec2.insert(4);
    // vec2.insert(5);
    // vec2.insert(6);

    // // vec2.insert(4);
    // // vec2.insert(5);
    // // vec2.insert(6);
    //     if !(&vec1 - &vec2).is_empty(){
    //         println!("Ok");
    //     }else{
    //         println!("Err");
    //     }


    // let mut hashmap: HashMap<i32, String> = HashMap::new();
    // hashmap.insert(1i32,"a".to_string());
    // hashmap.insert(2i32,"b".to_string());
    // hashmap.insert(3i32,"c".to_string());


    // let mut vector: 
    // let mut vector: Vec<Person> = hashmap.iter().map(|Person{id,name}| id name.clone()).collect();
    //     for a in hashmap.iter(){
    //         println!("{:?}",a);
    //     }
    // hashmap.remove("a");
    // // hashmap.remove(&2);
    // // hashmap.remove(&3);
    // println!("------------------");
    // for a in hashmap.iter(){
    //         println!("{:?}",a);
    //     }

    // let mut x: HashMap<i32, String> = HashMap::new();
    // x.insert( 1i32, "a".to_string());
    // x.insert( 2i32, "b".to_string(),);
    // // remove_hashmap(&mut x,2i32);

    // // println!("Now down to {:?}" , x);
    // for (key,value) in x.iter(){
    //     println!("key:      {}",key);
    //     println!("value:    {}",value);
    // }
    
    // println!("{:?}",vector);
    // vector.push("d".to_string());
    
        // for a in vector.iter(){
        //     println!("{}",*a+1);
        // }
    let mut p1 = Person{
        id: 1u32,
        name: "a".to_string(),
    };

    let mut p2 = Person{
        id: 2u32,
        name: "b".to_string(),
    };

    let mut p3 = Person{
        id: 3u32,
        name: "c".to_string(),
    };

    let mut p4 = Person{
        id: 4u32,
        name: "d".to_string(),
    };

    let mut vector: Vec<Person> = Vec::new();
    vector.push(p1);
    vector.push(p2);
    vector.push(p3);

    for i in 0..vector.len(){
        vector[i].output();
    }

    change(&mut vector);



    // vector.push(p4);
    // let mut vector1: Vec<&Person> = Vec::new();
        // for res in vector.iter(){
        //     vector1.push(res);
        // }
    println!("----------------------");
    // let n: u32 = 10;
    // let mut x = 1;

    // for i in 0..1{
    //     vector1.push(p4);
    // }
    // vector1.push(&p4);
    // vector.push(p4);
    // vector = convert(vector1);

    // for i in

    // vector.remove_item("d".to_string());

    // let mut vector1: Vec<Person> = Vec::new();
    // // vector1.push(p1);
    // // vector1.push(p2);
    // vector1.push(p3);
    // vector1.push(p4);
    // println!("vector: ");

    // for a in vector.iter(){
    //     println!("{:?}",a.output());
    // }
    // println!("vector1:  ");
    // for a in vector1.iter(){
    //     println!("{:?}",a.output());
    // }
    // vector = vector1;
    // println!("vector: ");
    // for a in vector.iter(){
    //     println!("{:?}",a.output());
    // }

    // let mut block = Block{
    //     data: &vector,
    // };

    // let mut vector1: Vec<&Person> = Vec::new();
    // // let mut a = block.data;
    // // for i in 0..a.len(){
    // //     a[i].output();
    // // }

    // for a in block.data.iter(){
    //     // a.output();
    //     vector1.push(a);
    // }

    // for i in vector1.iter(){
    //     i.output();
    // }

    // let mut vector: Vec<i32> = Vec::new();
    // vector.push(1);
    // vector.push(2);
    // vector.push(3);
    // vector.push(4);
    // let mut vector1: Vec<i32> = Vec::new();
    // vector1.push(1);
    // vector1.push(2);
    
    // vector.remove(vector1);

    // let mut v = vec![1,2,3,4,5,6];
    // for res in vector1.iter(){
        // vector.retain(|i| i.name == "a".to_string());
        // // println!("{:?}", vector);
        //     for v in vector.iter(){
        //         v.output();
        //     }
    // }

    // let mut vec = vec!["a", "b", "c", "d"];
    // for i in 0..vec.len(){
    //     vec.remove(0);
    // }
    // println!("{:?}",vec);
    // vec.remove_item(&"a");

    // let mut a = 1;
    // vector1.push(block.data[a]);

    // let mut vector: Vec<Person> = Vec::new();
    // let mut p1 = Person{id: 1,name: "a".to_string()};
    // let mut p2 = Person{id: 2,name: "b".to_string()};
    // let mut p3 = Person{id: 3,name: "c".to_string()};
    // // let mut vector = Vec::new();
    // vector.push(p1);
    // vector.push(p2);
    // vector.push(p3);
    // // let mut a = vector[0];
    // let mut t = 1;
    // let person = &vector[t];
   
    // let mut a = 10;
    //     let mut a = String::new();
    //     println!("input string: ");
    //     io::stdin().read_line(&mut a).unwrap();
    //     println!("inside scope: {}",a);
    // }
    // println!("outside scope: {}",a);
    // let mut value: i32;
    // value = 2;
    // {
    //     value = 1;
    //     println!("value: {}",value);
    // }
    // println!("value: {}",value);


    // let mut vector: Vec<i32>  = Vec::new();
    // vector.push(1);
    // vector.push(2);
    // vector.push(3);
    // vector.push(4);
    // println!("{:?}",vector);
    // for i in 0..vector.len(){
    //     vector.remove(0);
    //     println!("{:?}",vector);
    // }
        
        

    // let mut st1 = String::from("\"Hello\"");
    // println!("{}",st1);
    // st1 = st1.replace("\""," ");
    // println!("{}",st1);
    ///////////////////
    


    // st1 = st1.retain(|c| c != '\"');
    // st1 = st1.Trim(st1,"\"");
    // st1 = st1.trim("\"");
    // println!("{}",st1);

    // let mut st = String::new();
    // let mut vector = vec![242, 136, 98, 62, 231, 58, 22, 235, 78, 46, 81, 237, 64, 56, 71, 181, 52, 202, 70, 186, 29, 32, 98, 237, 100, 31, 233, 70, 246, 39, 247, 161];
    // st = str::from_utf8(&vector).unwrap();
    // for ch in 0..vector.len(){
    //     st.push(vec[i].as char);
    // }
    // println!("{}",st);
    // let st = str::from_utf8(&vector);
    // println!("{}",st);
    // let mut string = String::from_utf8(vector);
    // println!("{}",string);
    // let mut text = String::new();
    // println!("input text:   ");
    // io::stdin().read_line(&mut text).expect("Invalid input");
    
    // let mut vector: Vec<String> =  Vec::new();
    // let mut st = String::new();
    // let mut st1 = String::new();
    // if text.len() % 3 == 1{
    //     for ch in text.chars(){
    //         // st.push(ch);
    //         st1.push(ch);
    //         if st1.len() == 3{
    //         //     vector.push(st);
    //         //     st.clear();
    //             st = st1.clone();
    //             vector.push(st);
    //             st1.clear();
    //         }
    //     }
    // }
    // println!("{:?}",vector);
    


    // for ch in text.chars(){
    //     if let Some(digit) = ch.to_digit(10){
    //         vector.push(digit);
    //     }
    // }
    // println!("{:?}",vector);

    // }else{
    //     println!("Something went wrong!!!");
    // }
    // if text.len() %3 != 0{
    //     println!("Something went wrong");
    // }else{
    //     for ch in text.chars(){
    //         // st.push(ch);
    //         st1.push(ch);
    //         if st1.len() == 3{
    //         //     vector.push(st);
    //         //     st.clear();
    //             st = st1.clone();
    //             vector.push(st);
    //             st1.clear();
    //         }
    //     }
    // }
    
    
    // println!("{}",9%3);

    // if text.len()%3 == 1{
    //     println!("true");
    // }else{
    //     println!("false");
    // }
    
    
    // let mut st = String::new();
    // let mut text = String::new();
    // println!("input text:   ");
    // io::stdin().read_line(&mut text).expect("Invalid input");
    // let mut vector = Vec::new();
    //     if text.len() % 3 == 0{
    //         let mut j: i32 = 0;
    //         for i in 0..text.len()/3{
    //             st = text[j..j+2].to_string();
    //             vector.push(st);
    //             // j += 2;
    //         }
    //     }else{
    //         println!("Something went wrong!!!");
    //     }
    // println!("{:?}",vector);

//     let raw_string = r"rust
// header1,header2,header3
// r1v1,r1v2,r1v3
// r2v1,r2v2,r2v3";

    // let main_vec = raw_string.lines()
    //                          .map(|s| s.trim().split(',').map(String::from).collect::<Vec<String>>())
    //                          .collect::<Vec<Vec<String>>>();

    
    // let main_vec = raw_string.map(|s| s.trim().split(','.map(String::from)).collect::<String>;
    // print!("{:?}", main_vec);
    // print!("{:?}", main_vec);



    // let mut person1 = Person{
    //     id: 1u32,
    //     name: "sok".to_string(),
    // };
    // let mut person2 = Person{
    //     id: 2u32,
    //     name: "sav".to_string(),
    // };
    // let mut person3 = Person{
    //     id: 3u32,
    //     name: "chan".to_string(),
    // };

    // let mut per_vector: Vec<Person> = Vec::new();
    // per_vector.push(person1);
    // per_vector.push(person2);
    // per_vector.push(person3);

    // let mut search = String::new();
    // println!("Search for name:  ");
    // io::stdin().read_line(&mut search).expect("Invalid input");
    // // let mut i = 0;
    // let mut a = 0;
    //     for i in 0..per_vector.len(){
    //         if per_vector[i].name == search.trim(){
    //             a += 1;
    //             println!("See");
    //             per_vector[i].output();
    //             break;
    //         }
    //     }
    // if a == 0{
    //     println!("not found this name.");
    // }

    // per_vector[1].output();

    // let mut person1 = Person{
    //     id:     1,
    //     name:   "a".to_string(),
    // };
    // let mut person2 = Person{
    //     id:     2,
    //     name:   "b".to_string(),
    // };
    // let mut vector_person: Vec<Person> = Vec::new();
    // vector_person.push(person1);
    // vector_person.push(person2);

    // {
    // let mut block = Block{  
    //     data: &vector_person,
    // };
    // // block.output();
    // }
    // let mut person3 = Person{
    //     id:     3,
    //     name:   "c".to_string(),
    // };
    // // let mut vector_person.push(person3);
    // // let mut vector_person: Vec<Person> = Vec::new();
    // // mut vector_person: Vec<Person> = Vec::new();
    // vector_person.push(person3);
    // for value in &vector_person{
    //     // println!("{}",value);
    //     value.output();
    // }

    // let mut value: u32 = 12.0;
    // println!("value: {}",value);

    // println!("{}",public_value-1);
    
    // let mut result = add(1);
    // println!("{}",result);
    // let mut st = String::from("a");
    // let mut result = hash(&st);
    // println!("{:?}",result);

    



// fn main() {
    // This still presents a reasonable answer.
//     let twenty = multiply("10", "2");
//     print(twenty);

//     // The following now provides a much more helpful error message.
//     let tt = multiply("t", "2");
//     print(tt);
// // }


    // let mut result = return_result("lov".to_string());
    // println!("{:?}",result);

}
// As with `Option`, we can use combinators such as `map()`.
// This function is otherwise identical to the one above and reads:
// Modify n if the value is valid, otherwise pass on the error.
// fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
//     })
// }

// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         // Ok(n)  => println!("n is {}", n),
//         // Err(e) => println!("Error: {}", e),
//         Ok(n) => n,
//         Err(e) => e,
//     }
// }

fn add(value: i32) -> i32{
    value + 1
}
#[derive(Debug)]
struct A{
    a: i32,
}
// fn return_result(st: String) -> Result<A,String>{
//     let mut a: i32;
//     if st == "love".to_string(){
//         a = 10;
//         // return a;
//         // &a
//         // a.clone();
//         return Ok(A{
//             a,
//         })
//     }else{
//         // println!("Error Occur");
//         return Err(format!("error"))
//     }
//     // A{
//     //     a,
//     // }
// }
// pub type Hash = Vec<u8>;
// pub fn hash (data: &String) -> Hash {
//     crypto_hash::digest(crypto_hash::Algorithm::SHA256, da