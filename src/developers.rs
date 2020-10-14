use crate::gtrend::Since;
use select::document::Document;
use select::predicate::{Class, Name};
use serde::{Deserialize, Serialize};

const GITHUB_BASE: &str = "https://github.com";
const GITHUB_URL: &str = "https://github.com/trending/developers";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Developer {
    pub name: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
    pub sponsor_url: Option<String>,
    pub avatar: Option<String>,
    pub repo: Option<Repo>,
}

use crate::helpers;

fn select_data(html: &str) -> Vec<Developer> {
    let document = Document::from(html);
    let mut vec: Vec<Developer> = Vec::new();

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

        let name: Option<String> = node
            .find(Class("lh-condensed"))
            .next()
            .map(|x| escape(x.text()));

        let username: Option<String> = node
            .find(Class("lh-condensed"))
            .next()
            .and_then(|x| x.find(Name("a")).next())
            .and_then(|x| x.attr("href"))
            .and_then(|x| {
                let y = x.split("/").collect::<Vec<_>>();

                if y.len() > 1 {
                    Some(y[1].to_string())
                } else {
                    None
                }
            });

        let avatar: Option<String> = node
            .find(Name("img"))
            .next()
            .and_then(|x| x.attr("src"))
            .and_then(|x| {
                let y = x.split("?").collect::<Vec<_>>();

                if y.len() > 0 {
                    Some(y[0].to_string())
                } else {
                    None
                }
            });

        let repo_name: Option<String> = node.find(Class("h4")).next().map(|x| escape(x.text()));

        let url: Option<String> = username.clone().map(|x| format!("{}/{}", GITHUB_BASE, x));

        let sponsor_url: Option<String> = node
            .find(Class("mr-2"))
            .next()
            .and_then(|x| x.find(Name("a")).next())
            .and_then(|x| x.attr("href"))
            .map(|x| format!("{}{}", GITHUB_BASE, x));

        let repo_description: Option<String> =
            node.find(Class("mt-1")).next().map(|x| escape(x.text()));

        let repo_url: Option<String> = repo_name
            .clone()
            .and_then(|x| url.clone().map(|y| format!("{}/{}", y, x)));

        let repo: Option<Repo> = repo_name.map(|x| Repo {
            name: Some(x),
            description: repo_description.clone(),
            url: repo_url.clone(),
        });

        // println!("x {:?}", repo_url);

        let dev: Developer = Developer {
            name: name,
            username: username,
            url: url,
            sponsor_url: sponsor_url,
            avatar: avatar,
            repo: repo,
        };

        vec.push(dev);
    }
    vec
}

#[tokio::main]
pub async fn get_data(
    lang: Option<String>,
    since: Since,
) -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
    let since = since.to_str();

    let url = match lang {
        Some(l) => format!("{}/{}?{}", GITHUB_URL, l, since),
        _ => format!("{}?{}", GITHUB_URL, since),
    };

    let html = helpers::fetch_html(&url).await;
    let data: Vec<Developer> = match html {
        Ok(txt) => select_data(&txt),
        _ => {
            println!("err");
            Vec::new()
        }
    };
    // println!("{:?}", data);
    Ok(data)
}
