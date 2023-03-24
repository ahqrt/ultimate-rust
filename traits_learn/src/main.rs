use traits_learn::{notify, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("penguins win the stanley cup championship!"),
        location: String::from("pittsburgh, PA, USA"),
        author: String::from("Ice burgh"),
        content: String::from(
            "The pittsbursh penguins once again are the best \
             hockey team in the NHL.
            ",
        ),
    };

    // println!("1 new tweet {}", tweet.summarize());
    // println!("new article available! {}", article.summarize());

    notify(&tweet);
    notify(&article);
}
