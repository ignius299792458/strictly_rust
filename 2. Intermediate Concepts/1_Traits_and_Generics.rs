//  Traits (Similar to Interfaces)
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}) \ncontent: {}",
            self.headline, self.author, self.location, self.content
        )
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Function that accepts any type implementing Summary
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    // tweet
    let tweet = Tweet {
        username: String::from("Mr. Ignius"),
        content: String::from("I work hard by smartness!"),
    };
    tweet.summarize();
    tweet.default_summary();
    notify(&tweet);

    // news
    let news = NewsArticle {
        headline: String::from("Mr.Ignius done p2p"),
        location: String::from("127.0.0.1"),
        author: String::from("Mr. Gen"),
        content: String::from("Revolution of Whole Networking System!!"),
    };
    news.summarize();
    news.default_summary();
    notify(&news);
}
