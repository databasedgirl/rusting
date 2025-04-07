/*
PT:
    Escreva um programa que receba a digitação de um número inteiro e mostre qual é o seu número antecessor e o seu sucessor.
EN:
    Write a program that receives numbers and shows their predecessors and its successors.
*/

use std::io::Write; //should be used in order to make first print appear on screen
use std::io; // must be used so io::stdin() can be defined.

use std::num::ParseIntError;

fn main(){
    print!("Enter a number: ");
    std::io::stdout().flush().unwrap(); // Makes it possible to print out information on screen without need of user input
    let mut num: String = String::new(); //Reg user input
    
    io::stdin().read_line(&mut num).expect("Cannot read input."); //Store user input
    let num_conv:Result<i32,ParseIntError> = num.trim().parse(); //To convert successfully from string to int, should always have .trim() 
    let num_conv:i32 = match num_conv{ //Shadow the first declaration and turn it into a match to catch results.
        Ok(nc)=>nc,
        Err(_err)=>panic!("Only numbers are allowed."),
    };
    print!("\nOriginal: {}\nPrevious: {}\nNext: {}\n",num_conv,num_conv-1,num_conv+1); //Must convert String input back to integer. i32 architecture int;

}

//NOTES
//.unwrap() will unwrap the variable, making it "impossible" to shadow match 1st declaration for errors, thus making dev unable to catch. 
//Use .unwrap cautiously. 