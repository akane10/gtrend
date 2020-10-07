// use std::collections::HashMap;
// use scraper::Html;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const GITHUB_URL: &str = "https://github.com/trending?since=daily";

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

fn select_data(html: &str) -> Vec<String> {
    let document = Document::from(html);
    let mut vec: Vec<String> = Vec::new();

    for node in document.clone().find(Name("h1")) {
        let mut x = node.find(Name("a"));
        let push_to_vec = |y: &str| -> Option<()> {
            vec.push(y.to_string());
            Some(())
        };

        x.next().and_then(|y| y.attr("href")).and_then(push_to_vec);
        // match x.next() {
        // Some(xx) => vec.push(xx.attr("href").unwrap().to_string()),
        // None => (),
        // }
    }
    vec
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = fetch_html(GITHUB_URL).await;
    // let data = select_data(&html.unwrap());
    let data = match html {
        Ok(txt) => select_data(&txt),
        _ => Vec::new(),
    };
    println!("{:?}", data);

    Ok(())
}
