// use std::collections::HashMap;
// use scraper::Html;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const GITHUB_URL: &str = "https://github.com/trending";

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

fn select_data(html: &str) -> Vec<String> {
    let document = Document::from(html);
    let mut vec: Vec<String> = Vec::new();

    for node in document.clone().find(Name("h1")) {
        let push_to_vec = |y: &str| -> Option<()> {
            vec.push(y.to_string());
            Some(())
        };

        node.find(Name("a"))
            .next()
            .and_then(|y| y.attr("href"))
            .and_then(push_to_vec);
    }
    vec
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = fetch_html(GITHUB_URL).await;
    let data = match html {
        Ok(txt) => select_data(&txt),
        _ => Vec::new(),
    };
    println!("{:?}", data);

    Ok(())
}
