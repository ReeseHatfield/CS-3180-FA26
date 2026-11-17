fn main() {
    println!("Hello, world!");

    let vec1: Vec<&str> = vec![
        "Alice",
        "Bob",
        "Charlie",
    ];
    
    let vec2: Vec<i32> = vec![
        1, 2, 3
    ];

    let value: &str = first_or_default(vec1, "Derek");

}


fn first_or_default<T>(vec: Vec<T>, def: T) -> T {
    let maybe_first = vec.into_iter().next();
    match maybe_first {
        Some(first) => first,
        None => def,
    }
}