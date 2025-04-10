// PT: Faça um algoritmo que receba um número e mostre a tabuada dele
// EN: Create an algorithm that receives a number and shows its multiplication table 

use std::io::Write;
use std::io;
use std::num::ParseIntError;

fn main(){
    print!("Enter a number#> ");
    std::io::stdout().flush().unwrap();
    let mut num: String = String::new();
    let mut _i:i32 = 0;
    io::stdin().read_line(&mut num).expect("Only numbers allowed.");
    let convert_to_int: Result<i32,ParseIntError> = num.trim().parse();
    let convert_to_int: i32 = match convert_to_int{
        Ok(con2i) => con2i,
        Err(_err) => panic!("[X]: Only numbers allowed!"),
    };
    for i in 0..=10{
        println!("{}x{}={}", convert_to_int, i, convert_to_int*i);
    }

}