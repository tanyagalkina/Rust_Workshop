#![allow(unused_variables)]

fn main() {


let a = [1, 2, 3, 4, 5];
let b:[u16; 5] = [2, 3, 4, 5, 6];
let c = [0; 5]; // an array of five length with all the values of zero
println!("a, b, c: {:?} {:?} {:?}", a, b, c); //if printed without Debug Formatter will not go
let months = ["January", "February", "March", "April", "May", "June",
            "July", "August", "September", "October", "November", "December"];
let two = b[0];
let nov = months[10];
println!("two vars fomr the arrays are: {} {}", two, nov);            


}