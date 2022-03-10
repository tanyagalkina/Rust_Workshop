#![allow(unused_variables)]
#![allow(unused_parens)] // is done for compiler dont complain


//EXPRESSION AND STATEMENTS
fn main() {
    let a = 3 + 7;
    let b = (3 + 7); // (3 + 7) expression
    let c = { 3 + 7}; // Semicolons separate Statments
    let y  = {

        let mut x = 3;
        x = x * 2;
        x+ 1 // last things in the expression IS the value of the expression
    };
    println!("The value of y is: {}", y);

}