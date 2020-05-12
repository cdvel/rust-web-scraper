extern crate reqwest;
extern crate scraper;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use scraper::{Html, Selector};

fn main() {
    println!("\n{}\n", "*. Hacker news headlines");
    // assert!(hacker_news("https://news.ycombinator.com").is_ok());
    // assert!(hacker_news_headlines("https://news.ycombinator.com").is_ok());
    assert!(hacker_news_ranked("https://news.ycombinator.com").is_ok());
}

#[tokio::main]
async fn hacker_news_headlines(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    assert!(response.status().is_success());

    let body = response.text().await?;
    let fragment = Html::parse_document(&body);
    let stories = Selector::parse(".storylink").unwrap();

    for story in fragment.select(&stories) {
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{}", story_txt.join(" "));
    }

    Ok(())
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

#[tokio::main]
async fn hacker_news_ranked(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    assert!(response.status().is_success());

    let body = response.text().await?;
    let document = Document::from_read(body.as_bytes()).unwrap();

    for node in document.find(Class("athing")) {
        let rank = node.find(Class("rank")).next().unwrap();
        let story = node
            .find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        println!("{} {}", rank.text(), story);

        let subtext = node.next().ok_or("Err").unwrap();

        for field in &["score", "hnuser", "age"] {
            let value = subtext.find(Class(*field)).next().ok_or("Err");
            let value: String = if value.is_ok() {
                value.unwrap().text()
            } else {
                "".to_string()
            };
            print!("{}:\t\t{}\n", field, value);
        }

        let url = node
            .find(Class("title").descendant(Name("a")))
            .next()
            .unwrap();
        println!("url:\t\t{}\n", url.attr("href").unwrap());
    }

    Ok(())
}
