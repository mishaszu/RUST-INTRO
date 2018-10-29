use my_lib::summarize::Summary;
use my_lib::summarize::Tweet;
use my_lib::summarize::NewArticle;

fn traits_1() {
    let tweet = Tweet {
        username: String::from("misha_blue"),
        content: String::from("some not very long tweet content"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    let article = NewArticle {
        headline: String::from("some headline"),
        location: String::from("internet"),
        author: String::from("mishaszu blue"),
        content: String::from("some content"),
    };
    println!("1 new article: {}", article.summarize());
}

fn traits_2() {
    let tweet = Tweet {
        username: String::from("misha_blue"),
        content: String::from("some not very long tweet content"),
        reply: false,
        retweet: false,
    };
}

pub fn run() {
    println!("Running traits");
    traits_1();
    traits_2();
}
