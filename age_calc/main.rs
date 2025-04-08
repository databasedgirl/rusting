/*
    PT: 
        O algoritmo deve receber o ano de nascimento de uma pessoa e o ano atual. O algoritmo deve calcular e mostrar a idade da pessoa
    EN:
        Algorithm should receive year of birth & current year, then calculate person's age. 
*/  

use std::io::Write;
use chrono::Datelike;
use std::num::ParseIntError;

fn main(){
    print!("Year of birth> ");
    std::io::stdout().flush().unwrap();

    let mut year:String = String::new();
    std::io::stdin().read_line(&mut year).expect("Cannot read input.");

    let year: Result<i32, ParseIntError> =  year.trim().parse();
    let _year: i32 = match year{
        Ok(y)=>y,
        Err(_err)=>panic!("Only numbers are allowed"),
    };

    print!("{}\n",chrono::Utc::now().year()-_year);
}