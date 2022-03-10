#![allow(unused_variables)]

//LOOPS
fn main() {

    let mut counter = 0;
    let result = loop {
        counter +=1 ;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is {}", result);

    counter = 10;
    // in this case not evaluating anything
    while counter != 0 {
        println!("{}", counter);
        counter -= 1;
    }

    //FOR LOOPS ARE NOT LIKE C BUT RATHER LIKE PYTHON OR JAVASCRIPT
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {   // fist gets the elem form the array,
                                // then puts it in the loop body
        println!("I am {} years old", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
}