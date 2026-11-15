
struct User {
    active: bool,
    email: String,
    sign_in_count: u64,
}


struct Rectangle {
    width: i32,
    height: i32
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Rectangle {
    fn new(wid: i32, hgt: i32) -> Self {
        Rectangle{
            width: wid,
            height: hgt
        }
    }

    fn area(&self) -> i32 {
        self.width * self.height
    }

}


struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", 
        self.username, 
        self.content)
    }
}

impl SocialPost {
    fn new(content: String) -> Self{
        SocialPost {
            username: String::from("anonymous"),
            content: content,
            reply: false,
            repost: false
        }
    }
}


impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        // string format macro
        format!("{}, by {} ({})", 
        self.headline, 
        self.author, 
        self.location)
    }
}


// fn log_data(record: &dyn Summary){
//     println!("recorded: {}", record.summarize())
// }

fn log_data(record: &impl Summary){
    println!("recorded: {}", record.summarize())
}

fn main() {

    let rec = Rectangle::new(12, 15);

    let post = SocialPost::new(String::from("hello world"));

    log_data(&post);

    let article = NewsArticle{
        headline: String::from("test"),
        location: String::from("test"),
        author: String::from("test"),
        content: String::from("test"),
    };

    log_data(&article);

    post.summarize();


    println!("area was {}", rec.area());


    let bob = User {
        active: true,
        email: String::from("bob@gmail.com"),
        sign_in_count: 45
    };


    println!("bob's email {}", bob.email);

}
