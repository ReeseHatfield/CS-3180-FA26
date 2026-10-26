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

}


fn add(a: i32, b: i32) -> i32 {
    println!("add was called");

    a + b
}