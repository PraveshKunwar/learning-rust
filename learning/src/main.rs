#![allow(dead_code, unused_variables)]
use std::collections::HashMap;
fn main() {
    /*
        let mut x  = 5;
        println!("The value of x is {}", x);
        x = 6;
        println!("The value of x is {}", x);
        const MAX: i32 = 10000;
        println!("The max points you can get is {}", MAX);
        let y = x * 2;

        data types
        let a:  [i32; 5] = [1, 2, 3, 4, 5];
        println!("Values of a are {}", a[0]);
        let z: f32 = 1.0;
        let t = true;

        let f: bool = false;
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let tuple: (i32, f32, i32) = (1, 2.5, 3);
        println!("Adding the numbers, we got: {}", subtract(5, 3));

        if else
        let test = 7;
        if test < 5 {
            println!("{} is less than 5", test);
        } else if test > 5 {
            println!("{} is greater than 5", test);
        }
        loops
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);

        let mut num = 3;
        while num != 0 {
            println!("{}!", num);
            num -= 1;
        }
        let condition = true;

        let number = if condition { 5 } else { "six" };

        println!("The value of number is: {}", number);
        let arr: [i32; 5] = [10, 20, 30, 40, 50];
        for element in arr.iter() {
            println!("The value is {}", element);
        }
       println!("The value of converting 32 degrees far to cel is {}", far_to_cel(32.0));
       println!("Adding 5.2 and 3.3 gives me {}", add(5.2, 3.3));
    */
    //learn ownership
    /*Each value in Rust has a variable that’s called its owner.
    There can only be one owner at a time.
    When the owner goes out of scope, the value will be dropped.


        {                      // s is not valid here, it’s not yet declared
            let s: String = String::from("hello world");   // s is valid from this point forward

            // do stuff with s
        }                      // this scope is now over, and s is no longer valid

    //creating strings with String type

    let mut str = String::from("hello world");
    str.push_str(", my name is PraveshK!");
    println!("{}", str);

    let mut s1 = String::from("Hello test");
    let len = calc_length(&mut s1);
    println!("The length of '{}' is {}.", s1, len);


    let int_arr: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let slice = &int_arr[3..];
    println!("{:?}", slice); */

    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    let rect2 = Rectangle {
        width: 45,
        height: 45,
    };

    let thing = Rectangle::square(5);

    println!("Rect1 is {:?}", rect1);
    println!("Squared is {:?}", thing);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let home = IpAddr::V4(127, 0, 0, 1);
    let six = add_one(Some(5));
    let none = add_one(None);

    {
        let v: Vec<i32> = Vec::new();
    } // v goes out of scope here
    let mut t = vec![1, 2, 3];
    let first_element = &t[0];
    /*match t.get(1) {
        Some(second) => println!("The second element of the vector is {:?}", second),
        None => println!("No element at the position exists."),
    }*/
    for i in &mut t {
        *i += 50;
        println!("{}", i);
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "something";
    let s = data.to_string(); //this and String::from() are equal.
    let mut new_string = String::from("Hello");
    new_string.push_str(", world!");
    println!("{}", new_string);
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let ss = format!("{}-{}-{}", s1, s2, s3);
    let hello = "Здравствуйте";
    /*for c in hello.chars() {
        println!("{}", c);
    }*/
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Hello World"), 2);
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];
    let etc_scores: HashMap<_, _> = teams.into_iter().zip(init_scores.into_iter()).collect();
    let test_score = scores.get(&String::from("Hello World"));
    match test_score {
        Some(key) => println!("{}", key),
        None => println!("Nothing found"),
    }
    /*for (K, V) in &etc_scores {
        println!("{}: {}", K, V);
    }*/
    scores.entry(String::from("Red")).or_insert(50);
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    external: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_some_u8_value(num: i32) {
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn calc_length(s: &mut String) -> usize {
    //&String means its reffering to a string value
    return s.len();
}

fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    return x - y;
}

fn far_to_cel(f: f64) -> f64 {
    return (f - 32.0) * 0.5556;
}
