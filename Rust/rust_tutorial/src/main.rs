#![allow(unused)]

pub mod restaurant;

use rand::Rng;
use restaurant::order_food;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, *};
use std::io::{BufRead, BufReader, ErrorKind, Write};

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Add;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn say_hello() {
    println!("Hello World!");
}

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn sum_list(vec: &[i32]) -> i32 {
    let mut sum = 0;
    for v in vec {
        sum += v;
    }
    sum
}

fn get_2(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn print_str(x: String) {
    println!("A String {} ", x);
}

fn print_str_return(x: String) -> String {
    println!("A String {} ", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name)
}

fn main() {
    pub struct Bank {
        balance: f32,
    }

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt: f32) {
        let mut bank_ref: std::sync::MutexGuard<Bank> = the_bank.lock().unwrap();
        if bank_ref.balance < 5.00 {
            println!(
                "Current Balance : {} Withdrawal a smaller amount  ",
                bank_ref.balance
            );
        } else {
            bank_ref.balance -= amt;
            println!(
                "Customer withdraw : {} Current balance : {} ",
                amt, bank_ref.balance
            );
        }
    }
    fn customer(the_bank: &Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: &Arc<Mutex<Bank>> = &Arc::new(Mutex::new(Bank { balance: 20.00 }));

    let handles = (0..10).map(|_| {
        let bank_ref: Arc<Mutex<Bank>> = bank.clone();
        thread::spawn(move || customer(&bank_ref))
    });

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total {}", bank.lock().unwrap().balance);
    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //     the_bank.balance -= amt;
    // }

    // let mut bank = Bank { balance: 10000.89 };

    // withdraw(&mut bank, 47.89);

    // println!("Balance : {}", bank.balance);

    // fn customer(the_bank: &mut Bank) {
    //     withdraw(the_bank, 5.00);
    // }
    // thread::spawn(move || {
    //     customer(&mut bank);
    // })
    // .join()
    // .unwrap();

    // let t1 = thread::spawn(|| {
    //     for i in 1..25 {
    //         println!("Spawn thread : {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..20 {
    //     println!("Main thread : {} ", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // t1.join().unwrap();
    // #[derive(Debug)]
    // struct TreeNode<T> {
    //     pub left: Option<Box<TreeNode<T>>>,
    //     pub right: Option<Box<TreeNode<T>>>,
    //     pub key: T,
    // }

    // impl<T> TreeNode<T> {
    //     pub fn new(key: T) -> Self {
    //         TreeNode {
    //             left: None,
    //             right: None,
    //             key,
    //         }
    //     }

    //     pub fn left(mut self, node: TreeNode<T>) -> Self {
    //         self.left = Some(Box::new(node));
    //         self
    //     }

    //     pub fn right(mut self, node: TreeNode<T>) -> Self {
    //         self.right = Some(Box::new(node));
    //         self
    //     }
    // }

    // let node = TreeNode::new(15)
    //     .left(TreeNode::new(34))
    //     .right(TreeNode::new(45));

    // println!("{:?}", node);

    // let mut arr_it = [1, 2, 3, 4];
    // for val in arr_it.iter() {
    //     println!("{}", val)
    // }

    // let mut iter1 = arr_it.iter();

    // println!("1st : {:?}", iter1.next().unwrap());

    // let path = "line.txt";
    // let output = File::create(path);

    // let mut output = match output {
    //     Ok(file) => file,
    //     Err(error) => panic!("Cannot create file : {:?}", error),
    // };

    // write!(output, "Just some\nRandom words").expect("Failed to write to file");

    // let input = File::open(path).unwrap();
    // let buff = BufReader::new(input);

    // for line in buff.lines() {
    //     println!("{}", line.unwrap())
    // }

    // let output2 = File::create("Rand.txt");
    // let output2 = match output2 {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("Rabd.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Cannot create file : {:?} ", error),
    //         },
    //         _other_error => panic!("Ploblem"),
    //     },
    // };
    //order_food();

    // const PI: f32 = 3.141566;
    // trait Shape {
    //     fn new(length: f32, heigth: f32) -> Self;
    //     fn area(&self) -> f32;
    // }
    // struct Rectangle {
    //     length: f32,
    //     heigth: f32,
    // }
    // struct Circle {
    //     length: f32,
    //     heigth: f32,
    // }
    // impl Shape for Rectangle {
    //     fn new(length: f32, heigth: f32) -> Self {
    //         return Rectangle { length, heigth };
    //     }

    //     fn area(&self) -> f32 {
    //         return self.length * self.heigth;
    //     }
    // }
    // impl Shape for Circle {
    //     fn new(length: f32, heigth: f32) -> Self {
    //         return Circle { length, heigth };
    //     }

    //     fn area(&self) -> f32 {
    //         return (self.length / 2.0).powf(2.0) * PI;
    //     }
    // }

    // let rec: Rectangle = Shape::new(5.0, 6.0);
    // let cir: Circle = Shape::new(5.0, 6.0);

    // println!("Rectangle area : {}", rec.area());
    // println!("Circle area : {}", cir.area());

    // let mut heroes = HashMap::new();

    // heroes.insert("Superman", "Clark Kent");
    // heroes.insert("Batman", "Clark Kent");

    // for (k, v) in heroes.iter() {
    //     println!("{} = {} ", k, v)
    // }

    // if heroes.contains_key(&"Batman") {

    //     let the_batman = heroes
    // }
    // let mut str1 = String::from("Nikorn");
    // change_string(&mut str1);
    // let mut str3 = &str1;
    // println!("str3 = {} ", str3);
    // let binding = "Hello".to_string();
    // str3 = &binding;
    // println!("str3 = {} ", str3);

    //print_str(str1);

    // println!("Hello {} ", str1);
    // println!("Hello {} ", str2);

    // println!("{}", get_sum_gen(5, 4));
    // println!("{}", get_sum_gen(58.88, 4.0));

    // println!("{}", get_sum(5, 4));

    // println!("{:?}", get_2(5));

    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2 = vec![1, 2, 3, 4];
    // vec2.push(5);

    // println!("1st : {} ", vec2[0]);
    // let second = &vec2[1];
    // match vec2.get(1) {
    //     Some(second) => println!("2nd : {} ", second),
    //     None => println!("Mo 2nd value"),
    // }

    // for i in &mut vec2 {
    //     *i *= 2;
    // }
    // for i in &vec2 {
    //     println!("{}", i);
    // }

    // println!("Vec Length {} ", vec2.len());

    // println!("Pop : {:?} ", vec2.pop());

    // enum Day {
    //     Mondey,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday,
    // }
    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self {
    //             Day::Saturday | Day::Sunday => true,
    //             _ => false,
    //         }
    //     }
    // }

    // let today = Day::Mondey;

    // match today {
    //     Day::Mondey => println!("Everyone hates Monday"),
    //     _ => println!("Happy"),
    // }

    // println!("Is today the weekend {} ", today.is_weekend());

    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // let str3 = String::from("x r t b h k k a m c");

    // let mut vec1: Vec<char> = str3.chars().collect();

    // vec1.sort();
    // vec1.dedup();

    // for c in vec1 {
    //     println!("{}", c);
    // }

    // let str4 = "Ramdom string";
    // let mut str5 = str4.to_string();
    // println!("{}", str5);

    // let byte_arr = str5.as_bytes();
    // let str6 = &str5[0..6];

    // println!("String length : {}", str6.len());

    // str5.clear();

    // let str6 = String::from("Just some");
    // let str7 = String::from(" words");
    // let str8 = str6 + &str7;

    // for c in str8.bytes() {
    //     println!("{}", c);
    // }

    //rintln!("String length : {}", str6.len());

    // let mut str1 = String::new();
    // str1.push('A');
    // str1.push_str(" word");

    // for w in str1.split_whitespace() {
    //     println!("{}", w);
    // }

    // let str2 = str1.replace("A", "B");

    // println!("{}", str2);

    // let tuple = (47, "Nikorn".to_string(), 50_000_000);

    // println!("Name : {} ", tuple.1);

    // let (age, name, money) = tuple;

    // println!("Age : {} ", age);

    // let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // let mut idx = 0;

    // for val in arr_1.iter() {
    //     println!("Arr : {} ", val);
    // }

    // while idx < arr_1.len() {
    //     println!("Arr : {} ", arr_1[idx]);
    //     idx += 1;
    // }

    // loop {
    //     if arr_1[idx] % 2 == 0 {
    //         idx += 1;
    //         continue;
    //     }

    //     if arr_1[idx] == 9 {
    //         break;
    //     }
    //     println!("Val : {}", arr_1[idx]);
    //     idx += 1;
    // }

    // let age = 8;
    // let can_vote_age = 18;
    // let can_vote = match age.cmp(&can_vote_age) {
    //     Ordering::Equal => true,
    //     Ordering::Greater => true,
    //     Ordering::Less => false,
    // };
    // let mut my_age = 47;
    // let can_vote = if my_age >= 18 { true } else { false };
    //println!("Can vote : {}", can_vote);
    // const ONE_MILL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age = "47";
    // let mut age: u32 = age.trim().parse().expect("Age wasn't assiged a number");

    // age += 1;

    //println!("I'm {} and I want ${}", age, ONE_MILL);
    // println!("What is your name?");
    // let mut name = String::new();

    // let greeting = "Nice to meet you";

    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Don't receive input");

    // println!("Hello, {}!", name.trim_end());
}
