use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const GITHUB_URL: &str = "https://github.com/trending";

#[derive(Debug)]
struct Github {
    author: Option<String>,
    name: Option<String>,
    current_star: Option<String>,
    description: Option<String>,
    programming_language: Option<String>,
    url: Option<String>,
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

fn select_data(html: &str) -> Vec<Github> {
    let document = Document::from(html);
    let mut vec: Vec<Github> = Vec::new();

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

        let url: Option<String> = Some(format!(
            "{}/{}/{}",
            String::from("https://github.com"),
            username_reponame.clone().unwrap().0,
            username_reponame.clone().unwrap().1
        ));

        println!("x: {:?}", desc);
        let github: Github = Github {
            author: match username_reponame.clone() {
                Some(val) => Some(val.0),
                _ => None,
            },
            name: match username_reponame.clone() {
                Some(val) => Some(val.1),
                _ => None,
            },
            current_star: current_star,
            programming_language: lang,
            description: desc,
            url: url,
        };
        vec.push(github);
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
