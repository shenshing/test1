struct Person{
    id:     i32,
    name:   String,
}



// pub type vec_list = &'a Vec<Vector>;
// pub type vec_list<'a> = Vec<Vector<'a>>;
// pub type vec_list = Vec<Person>;

// struct Vector1<'a>{
//     vect_list:  &'a vec_list<'a>,
// }
// pub struct Total{
//     total: u32,
// }
use std::io;
// static vector: Vec<Block> = vec![];
// static vector: Vec<Vector1> = vec![];
// static vector : Vec<Vector1> = vec![];
// static vector: Vec<Vector1> = Vec::new();
// const vector: Vec<Vector1> = Vec::new();
// static NUM: i32 = 18;
// static vector: Vec<Person> = vec![];
// const vector1: Vector<'static> = Vector{
//     // vector: &vec![],
// };
// static mut a: Vec<Person> = Vec::new();
// struct Vector<'a>{
//     vector: &'a Vec<Person>,
// }
// pub type a<'a> =  Vec<Vector<'a>> = Vec::new();
// const VECTOR: Vector<'static> = Vector{
//     vector: &a,
// };

// struct Server<'a> {
//     foo: &'a str,
//     bar: i32,
// }
// const SERVER: Server<'static> = Server {
//     foo: "yay!",
//     bar: 1i32,
// };

// fn return_value<'a>(vec: &'a Server) -> &'a Server<'a>{
//     &SERVER
// }
use std::fs;
use std::fs::File;
use std::path::Path;
use std::prelude::*;
use std::fs::OpenOptions;
use std::io::{BufWriter, BufReader, Read, Write};
use std::io::BufRead;
use std::error::Error;



// use std::fs;
// use std::fs::File;
// use std::path::Path;
// use std::prelude::*;
// use std::io::{BufWriter, BufReader, Write, Read};
// use std::io::BufRead;
fn vec_to_str(vec: Vec<i32>) -> String{
    let mut st = String::new();
    for i in vec.iter(){
        st = st + &i.to_string();
    }
    st
}
fn main(){
    let mut vec = vec![0;64];
    println!("{:?}",vec);
    println!("{}",vec_to_str(vec));
    // let mut st = String::from("a");
    // // println!("{}",st);
    // let mut st1 = format!("{}\n",st);
    // println!("{}",st1);
    // // let mut path = Path::new("blockchain1.txt");
    // // let mut file = File::create(path);
    

    // // let mut st1 = String::new();
    // // file.read_to_string(&mut st1).unwrap();
    // // println!("{}",st1);
    // let mut file = match File::open(path){
    //     Err(why) => panic!("ERRROR: {}",why.description()),
    //     Ok(file) => file,
    // };
    // let reader = BufReader::new(file);
    // for line in reader.lines(){
    //     let line = line.unwrap();
    //     println!("{}",line);
    // }

    // let mut file = match File::open(path){
    //     Err(why) => panic!("ERRROR: {}",why.description()),
    //     Ok(file) => file,
    // };
    // let mut st = String::from("Hello");
    // let mut file = OpenOptions::new()
    //                 .write(true)
    //                 .append(true)
    //                 .open(path)
    //                 .unwrap();
    // file.write_all(st.trim().as_bytes());



    // let mut vector: Vec<Vec<String>> = Vec::new();
    // let mut s_vec: Vec<String> = Vec::new();
    // s_vec.push("s_vec a".to_string());
    // s_vec.push("s_vec b".to_string());
    // s_vec.push("s_vec c".to_string());
    // let mut s1_vec: Vec<String> = Vec::new();
    // s1_vec.push("s1_vec a".to_string());
    // s1_vec.push("s1_vec b".to_string());
    // s1_vec.push("s1_vec c".to_string());

    // vector.push(s_vec);
    // vector.push(s1_vec);

    // println!("{:?}",vector);

    // let mut vector1 = vec![vec![]];
    // let mut v = vec![];
    // v.push(1);
    // v.push(2);
    // v.reverse();
    // for i in v.iter(){
    //     println!("{}",i);
    // }
    // // v.push("string".to_string());
    // vector1.push(v);
    
    /*let mut path = Path::new("file.txt");
    // let mut file = File::create(path);

    /*let mut file = File::open(path);
    let mut id = String::new();
    let mut name = String::new();
    let mut vector = vec!["1","2","3","4"];
    
    let mut split_sign = String::from(",");
    let mut sign1 = String::from("[");
    let mut sign2 = String::from("]\n");
    let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();
    for i in 0..2{
        id.clear();
        name.clear();
        println!("input id: ");
        io::stdin().read_line(&mut id).unwrap();
        file.write_all(id.trim().as_bytes());
        file.write_all(split_sign.as_bytes());
        println!("input name:   ");
        io::stdin().read_line(&mut name).unwrap();
        file.write_all(id.trim().as_bytes());
        file.write_all(split_sign.as_bytes());
        file.write_all(sign1.as_bytes());
        for a in vector.iter(){
            file.write_all(a.as_bytes());
            file.write_all(split_sign.as_bytes());
        }
        file.write_all(sign2.as_bytes());
        //or you can input nothing inside to make new line in file
    }
    */
    let mut file1 = File::open(path);
    let  mut file1 = match File::open(path){
        Err(why) => panic!("ERROR:  {}",why.description()),
        Ok(file) => file,
    };
    let reader = BufReader::new(file1);
    for line in reader.lines(){
        // println!("{:?}",line);
        let line = line.unwrap();
        println!("{}",line);
        let mut vec1 = vec![];
        let mut vec2 = vec![];
        // let mut vector: Vec<_> = line.(split(',') || split('[')).collect();
        // println!("vector[0]:    {}",vector[0]);
        // println!("vector[1]:    {}",vector[1]);
        // println!("vector[2]:    {}",vector[2]);
        /*    for ch in line.chars(){
                if ch != ',' && ch != '[' && ch != ']'{
                    if vec1.len() < 2{
                        //after condition push to vec1
                        vec1.push(ch);
                    }else{
                        //after condition push to vec2
                        vec2.push(ch);
                    }
                }
            }
        println!("{:?}",vec1);
        println!("{:?}",vec2);*/
        for s in line.split(|x| (x == ',') || (x == '[') || (x == ']')) {
        // println!("{}", s);
            if vec1.len() < 2{
                vec1.push(s);
            }else{
                vec2.push(s);
            }
    }

    println!("vec1: {:?}",vec1);
    println!("vec2: {:?}",vec2);
    // println!("vec2 len() = {}",vec2.len());
    let mut vec3 = vec![];
        for v in vec2.iter(){
            if v != &""{
                // println!("result:   {}",v);
                vec3.push(v);
            }
        }
        println!("vec3: {:?}",vec3);
        for i in (0..vec3.len()).step_by(4){
            println!("vec[{}]:  {}",i,vec3[i]);
            println!("vec[{}]:  {}",i+1,vec3[i+1]);
            println!("vec[{}]:  {}",i+2,vec3[i+2]);
            println!("vec[{}]:  {}",i+3,vec3[i+3]);
            println!("-----------------");
        }
    }
    */
    // println!("{:?}",vec1);
    // vec.push(1);
    // println!("{:?}",vec);

    // let source = "this is a string | with two | delimeters";
    // let mut vec1 = vec![];
    // let mut vec2 = vec![];
    // for s in source.split(|x| (x != ',') || (x != '[') || (x != ']') ) {
    //     // println!("{}", s);
    //     if vec1.len() < 2{
    //         vec1.push(s);
    //     }else{
    //         vec2.push(s);
    //     }
    // }

    // println!("vec1: {:?}",vec1);
    // println!("vec2: {:?}",vec2);
    // let mut vector: Vec<_> = source.split(|x| (x == ' ') || (x == '|')).collect();
    // println!("{:?}",vector);





    

    


    // let mut vector: Vec<Person> = Vec::new();
    // let mut p1 = Person{
    //     id:     1i32,
    //     name:   "a".to_string(),
    // };
    // let mut p2 = Person{
    //     id:     2i32,
    //     name:   "b".to_string(),
    // };
    // let mut p3 = Person{
    //     id:     3i32,
    //     name:   "c".to_string(),
    // };
    // vector.push(p1);
    // vector.push(p2);

    // {
    //     let mut vector1 = &vector;
    // }
    // vector.push(p3);
    // vector.clear();

    // let mut x =10;
    // {
    //     println!("X = {}",x);
    // }
    
    
    
    // {
    //     let mut total = Total{
    //         total: 100u32,
    //     };
    // // }
    // {
    //     let mut total1 = &total;
    // }
    // let mut total2 = total;

    
    
    // println!("Total1:   {}",total1.total);

    // let mut choice = String::new();
    // println!("input your choice:    ");
    // io::stdin().read_line(&mut choice).unwrap();
    // match choice.trim().as_ref(){
    //     "1" => println!("inside 1"),
    //     "2" => println!("inside 2"),
    //     _ => println!("ERROR"),
    // }

    // let mut vector: Vec<i32> = Vec::new();
    // vector.push(1);
    // vector.push(2);
    // // vector.remove();
    // vector.clear();
    // println!("{:?}",vector);
    
    // let mut x = 10;
    // let mut x = 11;
    // println!("{}",x);

    // {
    //     x = 10;
    // }
    // println!("x = {}",x);

    // let mut a = Total{
    //     total:  100u32,
    // };
    // println!("Total =   {}",a.total);

    // a.total -= 10;
    // println!("Total =   {}",a.total);
    
    // let mut p1 = Person{
    //     id:     1i32,
    //     name:   "a".to_string(),
    // };
    // let mut p2 = Person{
    //     id:     2i32,
    //     name:   "b".to_string(),
    // };
    // let mut p3 = Person{
    //     id:     3i32,
    //     name:   "c".to_string(),
    // };
    // let  mut vector1: Vec<Person> = Vec::new();
    // vector1.push(p1);
    // vector1.push(p2);
    // vector1.push(p3);
    
    // let mut v1 = Vector{
    //     vector: &vector1,
    // };

    // vector.push(p1);
}