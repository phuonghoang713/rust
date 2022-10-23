use core::num;
use std::io::{self, stdin};

fn main() {
    println!("Hello, world!");
    loop{
        println!("input a Exit to exit.");
        println!("input a String: ");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.pop(); // \r\n   \n
        
        if line.eq("Exit") {
            break;
        }
        
        let mut s = String::new();
        loop {
            println!("input a Character: ");
            io::stdin().read_line(&mut s).unwrap();
            s.pop(); // \r\n   \n
            if s.len() == 1 {
                break;
            } else {
                println!("Your input is larger than 1 character. Try again");           
            }
        }
        
        let mut count:i32 = 0;
        let mut new_line = String::new();
        for c in line.chars() {
            let c1: String = c.to_lowercase().to_string();
            let s1: String = s.to_lowercase().to_string(); 
            if c1.eq(&s1) {
                count = count + 1;
            }
            else {
               new_line.push(c);
            }
        }
         println!("{}, {}", count, new_line);
        
    }

}
