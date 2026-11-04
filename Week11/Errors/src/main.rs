enum WritingUtensil {
    Pen,
    Pencil
}


fn divide(a: f64, b: f64) -> Result<f64, String> {

    if b == 0.0 {
        Err(String::from("Cannot divide by 0"))
    } else {
        Ok(a / b)
    }

}

fn run_division() -> Result<(), String>{
    let x = 10.0;
    let y = 5.0;

    let data = divide(x, y)?;

    println!("data was {data}");

    Ok(())

}

fn main() {

    let response = run_division();

    match response {
        Ok(()) => println!("success"),
        Err(err) => println!("something went wrong {err}"),
    }


//    let data =  match divide(x, y){
//         Ok(value) => {
//             value
//         },
//         Err(error) => {
//             println!("error was: {error}")
//         },
//     }


    let my_pencil = WritingUtensil::Pen;

    // match my_pencil {
    //     WritingUtensil::Pen => todo!(),
    //     WritingUtensil::Pencil => todo!(),
    // }
}
