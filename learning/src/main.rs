#![allow(unused_variables)]
fn main() {

    let mut x  = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    const MAX: i32 = 10000;
    println!("The max points you can get is {}", MAX);
    let y = x * 2;
    
    //data types
    let a:  [i32; 5] = [1, 2, 3, 4, 5];
    println!("Values of a are {}", a[0]);
    let z: f32 = 1.0;
    let t = true;

    let f: bool = false;
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tuple: (i32, f32, i32) = (1, 2.5, 3);
    println!("Adding the numbers, we got: {}", add(5, 3))
}

fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    return result;
}
