fn main() {
    println!("Hello, world!");


    let mut n = 0;
    let value = loop {
        n += 1;

        if n == 5 {
            break n * 2;
        }
    };

    println!("value was: {value}");

    let names = vec!["Alice", "Bob", "Charlie"];

    for name in &names {
        println!("name was {name}");
    }


    for i in 0..10 {
        println!("i {i}");
    }

    println!();
    println!();
    println!();

    let list = vec![1, 2, 3, 4, 5];
    
    let new_list: Vec<i32> = list.iter()
        .map(|i| i + 1)
        .map(|i|  i * 3)
        .map(|i| i / 2)
        .collect();

    println!("{:?}", new_list);


    let input: Option<&str> = Some("42 ");

    let doubled = input
        .map(|s| s.trim())
        .and_then(|s| s.parse::<i32>().ok())
        .map(|i| i * 2);

    println!("doubled was: {:?}", doubled.unwrap_or(0));

}
