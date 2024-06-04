use colored::*;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    let s1="hello";
    let s2=s1.clone();
    println!("{} == {}",s1,s2);
    println!("{}",s1.len());

    println!("Hello, world!");
    println!("Guessing game");
    println!("enter the number");
    //Generating the sceret number
    let secret:i32=rand::thread_rng().gen_range(1..101);
    println!(" secret number == {}",secret);
    //looping to make the user multiple chances to guess
    loop {
        let mut guess =String::new();
        io::stdin().read_line(&mut guess).expect("enter");
        let guess:i32=guess.trim().parse().unwrap();
        
        println!(" the number you've entered is == {}",guess);
        match guess.cmp(&secret){
            Ordering::Greater=>println!("{} Too big",guess.to_string().red()),
            Ordering::Less => println!("{} Too less",guess.to_string().red()),
            Ordering::Equal => {
                println!("You won");
            break;},
            
        }
    }
}


// //Concepts of ownership

// fn take_ownership(some_string: String) {
//     println!("Ownership of '{}' has been taken.", some_string);
//     // Here, some_string goes out of scope and is dropped.
// }

// fn take_copy(some_integer: i32) {
//     println!("Ownership of '{}' has been taken.", some_integer);
//     // Here, some_string goes out of scope and is dropped.
// }

// fn main() {
//     let my_string = String::from("hello");
//     let my_integer = 5;

//     take_ownership(my_string); // Ownership of the string is moved into the function
//     take_copy(my_integer); // copy of the integer is moved into the function

//     // Attempting to use my_string here would result in a compilation error
//     // because its ownership has been transferred to take_ownership()

//     println!("my_integer still usable: {}", my_integer);
//     println!("my_integer still usable: {}", my_string);

// }

// // when a string is passed to a function the function gets the ownership of the 
// // string once the function  finishes the execution the the string cannot be used
// // to overcome we instead of passing the string we pass the reference to the string 
// // by using the "&" whereas in case of integer this problem does not occur as
// // the copy is sent to the function instead of the variable itself