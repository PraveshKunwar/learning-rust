 #[allow(dead_code)]
 
 mod calculate {
   pub mod add_or_sub {
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        pub fn sub(x: i32, y: i32) -> i32 {
            x - y
        }
    }
    mod divide_or_multiply {
        pub fn divide(x: i32, y: i32) -> i32 {
            x / y
        }
        pub fn multiply(x: i32, y: i32) -> i32 {
            x * y
        } 
    }
}

use crate::calculate::add_or_sub;

fn thing (){
    add_or_sub::add(5, 6);
}