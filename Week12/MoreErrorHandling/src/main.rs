use std::{fs, io::read_to_string};

fn divide(num: f64, demon: f64) -> Option<f64> {
    if demon == 0.0 {
        None 
    } else {
        Some(num / demon)
    }
}

fn read_and_parse_number(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // can return Error
    let contents = fs::read_to_string(path)?;
    
    // can return Error
    let number: i32 = contents.trim().parse()?;

    Ok(number + 10)
}


fn parse_and_print(s: Option<&str>) -> i32 {

    let Some(parse_me) = s else {
        println!("None provided");
        return 0;
    };

    if let Ok(value) = parse_me.parse() {
        println!("{value}");
        return value;
    }

    0
}


fn main() {

    let a: Box<i32> = Box::new(45);

    // use deref if needed, Box will do this for you
    println!("Value of a was: {}", *a);
    println!("Value of a was: {}", a);

    let number = Some(7);
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    let letter: Option<i32> = None;

    // use else to specify failure case
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number");
    }



    let maybe_value: Option<i32> = Some(42);

    let result = maybe_value.ok_or("Value was missing"); 

    match result {
        Ok(num) => println!("Matched {num}"),
        Err(err) => println!("{err}")
    }


    let some_val: Option<&str> = Some("Rust");
    let none_val: Option<&str> = None;

    println!("{}", some_val.unwrap_or("Default"));
    println!("{}", none_val.unwrap_or("Default"));


    let d1 = "hi";

    let d1 = "2";

}
