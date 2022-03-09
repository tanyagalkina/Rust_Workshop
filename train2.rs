#![allow(unused_variables)]

//VARIABLES
fn main() {
    
    
    //mut
    let x = 42;
    //consider the thread: let mut x = 42; variables are immutable by default
    println!("The value of x is: {}", x);
    // x = 84;                      //this is called shadowing
    // println!("The value of x is: {}", x);
    
    let y = 117; // the previous values are deallocated when shadowing
    let y = y + 1;
    
    println!("The value of y is: {}", y);
    
    //shadow and change type
    let string = "STRING";
    println!("The value of stinrg is: {}", string);
    let string = string.len();
    println!("The value of stinrg is: {}", string);
    
    //consts
    
    const DECIMAL_CONST: u32 = 2022;

    //costs must be set only to an expression value, not a function result
    const FLOATING_POINT_CONST: f32 = 117.117 + 117.117;
//    const FUNCTION_CONST: i32 = const_function();
    
    //consts are not the same as immutable
    //consts are not just immutable by defaul, they are always immutable
    //must be annotated using "const" keyword
    
    println!("DECIMAL_CONST is {},\nFLOATING_POINT_CONST is {}", 
    //the convention is to name them Upper Case
    DECIMAL_CONST, FLOATING_POINT_CONST);
    
}
// fn const_function() {
//         42;
// }
