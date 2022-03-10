#![allow(unused_variables)]


//FUNCTIONS 
fn main() {
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(five());
    println!("The value of x is: {}", x);

    let if_five_is_odd = is_odd(5); 
    let if_six_is_odd = is_odd(6);
    println! ("Is 5 odd?: {} 6?: {}", if_five_is_odd, if_six_is_odd);

    let td: (f32, f64) = tuple_demo((4, 6,3));
    println!("Tuple demo: {:?}", td);

    println!("Factorial of five: {}", factorial(5));
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//checks the last bit by appliying AND
fn is_odd(x: i32) -> bool {

    if (x & 1) == 0 { false } //even
    else { true }

}

fn tuple_demo (t: (u8, u16, u32)) -> (f32, f64) {

    let x:f32 = t.0 as f32 + t.1 as f32;
    let y:f64 = t.2 as f64;
    (x, y)

}

fn factorial(num: u64) -> u64 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}





