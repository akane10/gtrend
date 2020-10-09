use crate::gtrend::Since;
use select::document::Document;
use select::predicate::{Class, Name};

const GITHUB_URL: &str = "https://github.com/trending/developers";

#[derive(Debug, Clone)]
pub struct Repo {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Developer {
    pub name: Option<String>,
    pub username: Option<String>,
    pub url: Option<String>,
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
            .and_then(|x| Some(escape(x.text())));

        let username: Option<String> = node
            .find(Class("lh-condensed"))
            .next()
            .and_then(|x| x.find(Name("a")).next())
            .and_then(|x| x.attr("href"))
            .map(|x| {
                let y = x.split("/").collect::<Vec<_>>();
                y[1].to_string()
            });

        let avatar: Option<String> = node
            .find(Name("img"))
            .next()
            .and_then(|x| x.attr("src"))
            .map(|x| {
                let y = x.split("?").collect::<Vec<_>>();
                y[0].to_string()
            });

        let repo_name: Option<String> = node
            .find(Class("h4"))
            .next()
            .and_then(|x| Some(escape(x.text())));

        let url: Option<String> = Some(format!(
            "{}/{}",
            String::from("https://github.com"),
            username.clone().unwrap(),
        ));

        let repo_description: Option<String> = node
            .find(Class("mt-1"))
            .next()
            .and_then(|x| Some(escape(x.text())));

        let repo_url: Option<String> = repo_name.clone().and_then(|x| {
            let u = format!("{}/{}", url.clone().unwrap(), x);
            Some(u)
        });

        // println!("x: {:?}", repo_url);
        let repo: Option<Repo> = match repo_name {
            Some(val) => {
                let r = Repo {
                    name: Some(val),
                    description: repo_description.clone(),
                    url: repo_url.clone(),
                };
                Some(r)
            }
            _ => None,
        };

        let dev: Developer = Developer {
            name: name,
            username: username,
            url: url,
            avatar: avatar,
            repo: repo,
        };

        vec.push(dev);
    }
    vec
}

#[tokio::main]
pub async fn get_data(
    lang: Option<&str>,
    since: Since,
) -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
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
