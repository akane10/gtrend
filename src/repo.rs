use select::document::Document;
use select::predicate::{Attr, Class, Name};

use crate::helpers;

const GITHUB_URL: &str = "https://github.com/trending";

#[derive(Debug)]
pub struct Repository {
    pub author: Option<String>,
    pub name: Option<String>,
    pub current_star: Option<String>,
    pub description: Option<String>,
    pub programming_language: Option<String>,
    pub url: Option<String>,
    pub stars: Option<String>,
    pub forks: Option<String>,
}

pub enum Since {
    Daily,
    Weekly,
    Monthly,
}

fn select_data(html: &str) -> Vec<Repository> {
    let document = Document::from(html);
    let mut vec: Vec<Repository> = Vec::new();

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

        let stars_forks: Vec<String> = node
            .find(Class("muted-link"))
            .map(|x| escape(x.text()))
            .collect::<Vec<_>>();

        // println!("x: {:?}", stars_forks);
        let repo: Repository = Repository {
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
            stars: Some(stars_forks[0].clone()),
            forks: Some(stars_forks[1].clone()),
        };
        vec.push(repo);
    }
    vec
}

#[tokio::main]
pub async fn get_data(
    lang: Option<&str>,
    since: Since,
) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let since = match since {
        Since::Daily => "daily",
        Since::Weekly => "weekly",
        Since::Monthly => "monthly",
    };

    let url = match lang {
        Some(l) => format!("{}/{}?{}", GITHUB_URL, l, since),
        _ => format!("{}?{}", GITHUB_URL, since),
    };

    let html = helpers::fetch_html(&url).await;
    let data: Vec<Repository> = match html {
        Ok(txt) => select_data(&txt),
        _ => {
            println!("err");
            Vec::new()
        }
    };
    // println!("{:?}", data);
    Ok(data)
}
