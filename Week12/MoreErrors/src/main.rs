use core::num;
use std::{error::Error, fs};


fn divide(num: f64, denom: f64) -> Option<f64> {
    if denom == 0.0 {
        None
    }
    else {
        Some(num / denom)
    }
}


fn read_and_parse_number(path: &str) -> Result<i32, Box<dyn Error>> {
    // can return Error
    let contents = fs::read_to_string(path)?;
    
    // can return Error
    let number: i32 = contents.trim().parse()?;
    
    Ok(number + 10)
}


fn print(s: Option<&str>){

    let Some(parse_me) = s else {
        println!("None provided");
        return;
    };

    println!("{parse_me}");


}

fn main() {

    // let a: Box<i32> = Box::new(45);
    let a = Box::new(45);
    println!("Value of a was: {}", a);

    // println!("Value of a was: {}", *a);

    match divide(10.0, 0.0){
        Some(value) => println!("value was {value}"),
        None => println!("you got none back"),
    };


    let number = Some(7);

    if let Some(i) = number {
        println!("number was {i}");
    }

    // let res = maybe.ok_or("Error: missing value");

    let maybe: Option<&str> = Some("hello world");
    // let maybe: Option<&str> = None;

    let value = maybe.unwrap_or("Default value");

    println!("Maybe was: {:?}", value);








}
