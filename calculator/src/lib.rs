#![allow(dead_code)]



 mod calculator {
    pub mod add_or_sub {
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        pub fn sub(x: i32, y: i32) -> i32 {
            x + y
        }
    }
    pub mod divide_or_multiply {
        pub fn divide(x: i32, y: i32) -> i32 {
            x / y
        }
        pub fn multiply(x: i32, y: i32) -> i32 {
            x * y
        }
    }
    pub struct Area {
        pub width: u128,
        pub height: u128,
    }
    impl Area {
        pub fn calc(dimensions: &Area) -> u128 {
            dimensions.height * dimensions.width
        } 
    }
}

pub fn test() {
     calculator::add_or_sub::add(5, 6);
    let dim1 = calculator::Area {
        width: 256,
        height: 256
    };
    println!("The dimensions of width {:?} times height {:?} is equal to area of {:?}.", dim1.width, dim1.height, calculator::Area::calc(&dim1));
    }


 