#![allow(unused_variables)]

//TYPES
fn main() {
    //Signed integers i8, i16, i32, i63, i128, isize   ==> this is the size of the architecture
    //Unsigned integers u8, u16, u32, u63, u128, usize   

    let x128: u128 = 0xFAFBFCFD_FEF1F2F3_F4F5F6F7_F8F9FAFB;//underscores are ignored
    let x64: i64 = 123456;

    let x = 2.0; // f64 is the default
    let y: f32 = 3.0;
    println!("The value of x128 is: {}", x128);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let hello = String::from("שָׁלוֹם");
    println!("This is juwish Hello: {}", hello);

    
    //https://unicode-table.com/en/sets/top-emoji/
    // characters and booleans
    //UTF-8    !!!!
    let c = '😻';
    let z = '😁';
    let a = false;
    let cry = '😂';
    println!("{}  &&  {} && {}", z, c, a);
    let a = true;
    println!("{} {} !!", a, cry);
}