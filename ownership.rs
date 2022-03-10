fn main() {
    
    let a = 7;
    let mut b = a;
    b = b + 1;
    println!("a = {}, b = {}", a, b);
    
    //strings
    
    let str1 = String::from("What is move ownership?");
    //In the next line what you want to do?
    //a) Do you want two copies of the string?
    //b) Or, do you want one copy and two variables pointing to it?
    
    //let mut str2 = str1;   / does not compile
    
    //a)for copiying the string one have to use the clone(); function
    //let str2 = str1.clone();
    
    //b) if we want 1 copy and 2 variables pointing to it, then
    // Each bit of data, e.g. "What is move ownership" has an owner
    //and there can be only one
    // now str1 is the owner of data "What is move ownership"
    
    let str2 = &str1; //by reference, means sthat both vars now pointing to the same String
    let str3 = &str2; // read only!!
    println!("str1 = {}, str2 = {}", str1, str2);
    println!("str3 = {}", str3);
    
    
    //MUTABLE BORROW
    // there is only one reference pointing to a data at a certain time
    let mut string1 = String::from("Hello");
    let string2 = &mut string1; //mutable borrow line
    string2.push('!');  //one char ( cause it is aLInked list??)
    println!("string2 = {}", string2); // ptinted before the string1 ..
    //println!("string1 = {}", string1); // we can only use the original
    // value by the time the borowed value is dropped         
    // (means went out of scope) 
    string2.push('?');  // means that the compiler looks ahead 
    
    //borrowing with function calls
    
    let mut string = String::from("Please, add a dot");
    append_dot(&mut string);
    println!("string with dot = {}", string);

    //primitive values
    let mut n : i32 = 19;
    plus_one_by_ref(&mut n); //borrowing stops here
    println!("n = {}", n);

}

//which primitive types implement copy traits...


fn append_dot(t: &mut String) {

    t.push('.');
}

fn plus_one_by_ref(n: &mut i32) {

    *n = *n + 1;
}
