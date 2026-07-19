use core::str;

const PI: f32 = 3.14159265358979323846264338327950288; // constant, type must be declared

// Entry point
fn main() {
    let x: i32 = 32;
    let y: u32 = 100;
    let pickle: &str = "pickles!";

    let f = 6.9; // float, can explicitly declare type, or can leave it to compiler
    let mixtape = ("Wowowow", 67, "Dude"); // tuple

    let num_slice = &[1,2,3,4];

    println!("Hello, world! {} {} {} {:?} {} {:?}", x, pickle, f, mixtape, y, num_slice);
    println!("The sum of {} and {} is {}", x, y, add(x, y as i32)); 
    // 'as i32' is needed to convert y to i32, since add() takes two i32s
}

fn add(x: i32, y: i32) -> i32 { // '-> i32' declares return type.
    return x + y;
}

