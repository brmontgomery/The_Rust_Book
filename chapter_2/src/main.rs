//this is how to import libraries in rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

//main function ust like C++
//function called with fn. Not sure if we can give a return type yet.
fn main() {
    //println is a macro
    //macros are code that is only expanded at runtime
    //I.E. if I have a macro returnNum!(2+3)
    //but earlier I had defined the macro as multiplying its input by 5 (returnNum(x) {5+x})
    //The resultant code would end up returning the result of 5 * 2 + 3 rather than 5 * (2+3) because that is how it is expanded at runtime.
    println!("Guess the number!");

    //most interesting thing here is the range expression in form (start..=end). It's inclusive on lower and upper bounds.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //let says that we are creating a variable (immutable by default - value won't change, not const though)
    //using mut allows us to change the variable
    //: after the variable name allows us to assign a type to the variable - VSCode did it automatically for me...
    //:: works just like C++

    loop {
        let mut guess = String::new();

        println!("Please input your guess.");

        //io is part of RUST "standard library"
        io::stdin()
            //reads theline and passes the variable guess to store the input in.
            //references are also immutable by default.
            .read_line(&mut guess)
            //error handling
            //read_line is an enum that can pass back error info if there's an issue.
            //If we get read back an error, return this error message
            .expect("Failed to read line.");

        //this is technically a different cariable since it has a different type.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //acts similar to C++. The brackets let you output a var into the string at runtime.
        //can also do the c++ comma separated list after the string.
        println!("You guessed: {guess}.");
        
        //match seems a lotlike select statements.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!:"),
            Ordering::Greater => println!("Too big!:"),
            Ordering::Equal => {
                println!("You win!:");
                break;
            },
        }
    }
}
