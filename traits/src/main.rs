use traits::{NewsArticle, Summary, Tweet}; 
fn main() {
    let tweet = Tweet {
        username: String::from("Santosh"),
        content: String::from("Hello there"),
        reply: true,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Launch of new AI model"),
        location: String::from("Asia, china"),
        author: String::from("Santosh"),
        content: String::from(
            "This is Santosh and found out that new AI model was launched \
            in china and is globally abailable  it is free and opensource."
        ),
    };
    println!("New article available! {}", article.summarize());
}
