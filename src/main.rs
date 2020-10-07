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

    for node in document.clone().find(Class("Box-row")) {
        let escape = |str_: String| -> String {
            str_.split_ascii_whitespace()
                .fold(String::new(), |acc, val| {
                    if acc.is_empty() {
                        val.to_string()
                    } else {
                        format!("{} {}", acc, val)
                    }
                })
        };

        let username_reponame: Option<(String, String)> = node
            .find(Name("h1"))
            .next()
            .and_then(|x| x.find(Name("a")).next())
            .and_then(|x| x.attr("href"))
            .map(|x| {
                let y = x.split("/").collect::<Vec<_>>();
                (y[1].to_string(), y[2].to_string())
            });

        let current_star: Option<String> = node
            .find(Class("float-sm-right"))
            .next()
            .and_then(|tag| Some(escape(tag.text())));

        let lang: Option<String> = node
            .find(Attr("itemprop", "programmingLanguage"))
            .next()
            .and_then(|x| Some(x.text()));

        let desc: Option<String> = node
            .find(Name("p"))
            .next()
            .and_then(|x| Some(escape(x.text())));

        println!("x: {:?}", current_star);
    }
    vec
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = fetch_html(GITHUB_URL).await;
    let data = match html {
        Ok(txt) => select_data(&txt),
        _ => {
            println!("err");
            Vec::new()
        }
    };
    println!("{:?}", data);
    Ok(())
}
