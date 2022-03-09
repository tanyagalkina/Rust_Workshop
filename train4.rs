#![allow(unused_variables)]

//STRINGS
//There are tow kinds of Strings in Rust ( static and dynamic)
//behind the seens the first one is like an immutable array(str), 
//and the second like linked list(String::from("someString"))
//similar like in Haskell, where you cann access and modify each element

fn main() {

    //str static strings UTF-8, also called string literals
    //we can build a Struct String from the str literal
    //let emoji = "😂"; 
    let emoji = "💙";   
    let tanya = "tanya 😻"; // 6 length?
    println!("Some str stuff: {} {}", tanya.len(), tanya.is_empty());
    println!("The bytes of tanya: {:?}", tanya.as_bytes());
    println!("The bytes of emoji are: {:?}", emoji.as_bytes());


    //Rust LangRef   Struct String
    //https://doc.rust-lang.org/std/string/struct.String.html
    //List Strings
    let mut hello = String::from ("Hello");
    hello.push('R');
    hello.push_str("ust!");

    println!("Hello_String: {} ", hello);
    hello.insert(5, ',');
    hello.insert(6, ' ');
    println!("Hello_String: {} ", hello);

    let sparkle_heart = vec![240, 159, 146, 150];
    let blue_heart = vec![240, 159, 146, 153];

    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    //using shadowing
    let blue_heart = String::from_utf8(blue_heart).unwrap();

    println!("{}", sparkle_heart);
    println!("{}", blue_heart);
    //assert_eq!("💖", sparkle_heart);


}