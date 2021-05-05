#![allow(dead_code, unused_variables)]

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

    let person = User {
        name: String::from("Pravesh Kunwar"),
        email: String::from("testtest1234@gmail.com"),
        dob: String::from("01/01/04"),
        age: 17
    };

   

    println!("Hello {}!", person.name);

    let rect1  = Rectangle {
        width: 30,
        height: 30
    };

    let rect2 = Rectangle {
        width: 45,
        height: 45
    };

    let thing = Rectangle::square(5);

    println!("Rect1 is {:?}", rect1);
    println!("Squared is {:?}", thing);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
struct User {
    name: String,
    email: String,
    dob: String,
    age: i32
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
            height: size
        }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}


fn calc_length(s: &mut String) -> usize { //&String means its reffering to a string value
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

