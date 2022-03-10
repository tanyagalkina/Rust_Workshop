fn main() {
    let mut words = vec![ String::from("Hello"),
                          String::from("Yellow"),
                          String::from("Rust"),
                          String::from("War"),
                          String::from("Elm"),
                          String::from("Bash"),
                          String::from("The_end"),];
println!("{:?}", words);
let t = words[1].clone(); //if we would borrow we could only borrow
                          // only once from the ame data structure
words[1] = words[2].clone();
words[2] = t;
println! ("{:?}", words);
//to avoid "cannot borrow as mutable more that once at a time"
words.swap(3, 5);
println! ("{:?}", words);

//STACK TYPES WHICH IMPLEMENT THE COPY TRAIT
//bool
//chracter
//numbers
//slices
//fix sized arrays
//tuples containing primitives
//function pointers

let a = 42;
let a = true;

let b = a;
println!("This is a 'b' binding which has got the type copied from a: => {}", b);
println!("This is an 'a' binding which has been printed anywhere\ncause it implement the copy trait: => {}", a);
println!("This is a 'b' binding which has got the type copied from a: => {}", b);

//I WILL EAT YOU cause the String is not a primitive type
// let a = String::from("Do you understand now the borrowing and ownership?");
// println!("This is my question: {}", a);
// i_will_eat_you(a);
//println!("This is my question: {}", a); // value borrowed here after move

//I WILL NOT EAT YOU  cause the String is of a primitive type
i_will_not_eat_you(a);
println!("This is my answer: {:?}", a); // 

}

fn i_will_eat_you(string: String) -> String { string }

fn i_will_not_eat_you(is_it: bool) -> bool { is_it }

 