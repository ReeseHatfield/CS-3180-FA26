
fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("some new data");

}

fn main() {

    println!("Hello, world!");

    // binding
    // create a binding between label (foo) and the data (56)
    // mut modifies the label to be able to able to be changed
    // let mut foo: i32 = 56;
    let mut foo = 56;

    foo = 35;

    let bar = 2;

    println!("foo was {foo} and bar was {}", bar);

    let result = add(36, 35);
    println!("result was {}", result);



    let y = {
        let x = 4;
        x + 1
    };

    println!("y was {y}");


    let mut s: String = String::from("Hello,");
    let literal: &str = "world!";
    s.push_str(literal);

    let slice: &str = &s[1..3];
    println!("{slice}");

    println!("{s}");


    let s = String::from("Hello world");

    let length = calculate_len(&s);

    // println!("{s}");


    let mut temp = String::from("Reese Hatfield");
    change(&mut temp);

    println!("{temp}");



    // let bind_two = bind_one.clone();

    // println!("{bind_one}");
    // println!("{bind_two}");

    // foo(bind_one);


    let a = 5;
    let b = 6;

    let _ = add(a, b);

    println!("a = {a}, b = {b}");

}



fn add(a: i32, b: i32) -> i32 {
    println!("add was called");

    a + b
}