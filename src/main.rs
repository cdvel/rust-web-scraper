extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;

fn main() {
    println!("{:?}", hacker_news("https://news.ycombinator.com"));
}

#[tokio::main]
async fn hacker_news(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    assert!(response.status().is_success());

    let body = response.text().await?;
    Document::from_read(body.as_bytes())
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    
    Ok(())
}
