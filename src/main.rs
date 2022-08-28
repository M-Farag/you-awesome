use std::io;

fn main(){
    println!("write your name :) !!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Err");

    println!("You\'re AWESOME {}",name);
}