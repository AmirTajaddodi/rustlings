use std::io;
//rust brings a bunch of libraries in advance for you which are called prelude 

fn main() {
    println!("Guess the number !");

    println!("Please input your game");

    let mut guess = String::new();
    //if we didnt import the std::io library what we could have done instead was just go std::io like cpp
    i0::stdin()
    .readline(&mut guess)
    //readline is a method of the stdin function passing guess as the argument saying where the value should be stored
    //this needs to mutable to allow the readline function to change the value of guess
    //references are immutabe by default so u need to make sure that you invoke &mut Guess
    .expect("Failed to readline");

    println!("You guessed {guess}");
}
