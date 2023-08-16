#![allow(non_snake_case)]
use std::io::stdin;
fn main() {
    println!("Age calculator");
    loop {
        println!("Please type in your age");
        let mut age = String::new();
        stdin()
            .read_line(&mut age)
            .expect("Failed to read line");
        let age_number = match age.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let days = calcAge(age_number);
        println!("You have lived for {days}");
    }
}
fn calcAge(age :u16) -> u16 {
    age*365
}