use core::num;



#[derive(Debug, PartialEq)]
enum State {
    Alabama,
    Arizona,
    Ohio,
    Wyoming,
}


#[derive(Debug, PartialEq)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

// fn get_value(c: &Coin) -> i32 {

//     match c {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }


fn trade (c: Coin) -> Coin {
    Coin::Penny
}

fn main() {
    // let c1: Coin = Coin::Penny;
    let c2: Coin = Coin::Quarter(
        State::Ohio
    );

    if c2 == Coin::Nickel {
        println!("You have 5c");
    }

    let new_coin = trade(c2);

    println!("{:?}", new_coin);


    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => {
            println!("you matched true");

            1
        },
    };

    println!("value was {binary}");



    

    // let change: Vec<Coin> = vec![
    //     c1,
    //     c2
    // ];

    let number = -1;

    match number {
        1 => println!("You value was 1"),
        2 | 3 => println!("Your value was either 2 or 3"),
        13..=19 => println!("Your value was a teen number"),
        _ => println!("your number is not special"),
    };


    let c3 = Coin::Quarter(
        State::Alabama
    );

    match c3 {
        Coin::Penny => todo!(),
        Coin::Nickel => todo!(),
        Coin::Dime => todo!(),
        Coin::Quarter(state) => {
            match state {
                State::Alabama => todo!(),
                State::Arizona => todo!(),
                State::Ohio => todo!(),
                State::Wyoming => todo!(),
            }
        },
    }



    // println!("{}", get_value(&c3));



}
