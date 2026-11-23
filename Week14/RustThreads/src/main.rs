use std::{thread, time::Duration};


async fn get_size() -> usize {
    10
}
    
async fn fetch_count() -> usize {
    println!("hello");
    
    42 + get_size().await

}
    

async fn say_world() {
    println!("world");
}

#[tokio::main]
async fn main() {
    // since this is a compiler generated type
    // have to let type inference take the wheel
    let op = say_world();
	 
    // this will come first
    println!("hello");
    
    // calling await will execute it
    op.await;
}

// fn main() {
//     // println!("Hello, world!");

//     // let handle = thread::spawn(|| {
//     //     for i in 1..5 {
// 	//         println!("Worker thread: {}", i);
	 
// 	//         thread::sleep(Duration::from_millis(500));
// 	//     }

//     //     42
//     // });

//     // println!("HELLO");

//     // let value = handle.join().unwrap();
//     // println!("{value}");

// }
