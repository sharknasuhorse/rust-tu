use toreit::Tweet;
use toreit::Summary;
use toreit::NewArticle;

fn main() {
    let tweet = Tweet{
        username: String::from("nasu"),
        content: String::from("kuso tweet"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewArticle {
        headline: String::from("hi"),
        location: String::from("JP"),
        author: String::from("nasu"),
        content: String::from("hihihihi"),
    };
    println!("{}", article.summarize());
}
