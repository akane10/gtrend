use select::document::Document;
use select::predicate::{Attr, Class, Name};

use crate::gtrend::Since;
// use crate::gtrend::Since::*;
use crate::helpers;
use serde::{Deserialize, Serialize};

const GITHUB_URL: &str = "https://github.com/trending";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildBy {
    pub username: Option<String>,
    pub href: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub author: Option<String>,
    pub name: Option<String>,
    pub current_star: Option<i32>,
    pub description: Option<String>,
    pub programming_language: Option<String>,
    pub url: Option<String>,
    pub stars: Option<i32>,
    pub forks: Option<i32>,
    pub build_by: Vec<BuildBy>,
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

        let current_star: Option<i32> = node.find(Class("float-sm-right")).next().and_then(|tag| {
            let x = tag.text();
            let s: Vec<_> = x.split_whitespace().collect();
            Some(s[0].parse::<i32>().unwrap())
        });

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

        let stars_forks: Vec<i32> = node
            .find(Class("muted-link"))
            .map(|x| {
                let s = escape(x.text()).replace(",", "");
                s.parse::<i32>().unwrap()
            })
            .collect::<Vec<_>>();

        let build_by: Vec<BuildBy> = node
            .find(Class("avatar-user"))
            .map(|x| {
                let username: Option<String> = x.attr("alt").map(|val| {
                    let u: Vec<&str> = val.split("@").collect();
                    u[1].to_string()
                });
                let avatar = x.attr("src").map(|a| a.to_string());
                let href = format!("{}/{}", "https://github.com", username.clone().unwrap());

                let build_by = BuildBy {
                    username: username,
                    avatar: avatar,
                    href: Some(href),
                };

                build_by
            })
            .collect::<Vec<_>>();

        // println!("x: {:?}", build_by);
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
            build_by: build_by,
        };
        vec.push(repo);
    }
    vec
}

#[tokio::main]
pub async fn get_data(
    lang: Option<&str>,
    since: Since,
    spoken_lang: Option<&str>,
) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let since = match since {
        Since::Daily => "daily",
        Since::Weekly => "weekly",
        Since::Monthly => "monthly",
    };

    let url = match (lang, spoken_lang) {
        (Some(l), Some(sl)) => format!(
            "{}/{}?since={}&spoken_language_code={}",
            GITHUB_URL, l, since, sl
        ),
        (Some(l), None) => format!("{}/{}?since={}", GITHUB_URL, l, since),
        (None, Some(sl)) => format!("{}?since={}&spoken_language_code={}", GITHUB_URL, since, sl),
        _ => format!("{}?since={}", GITHUB_URL, since),
    };

    let html = helpers::fetch_html(&url).await;
    let data: Vec<Repository> = match html {
        Ok(txt) => select_data(&txt),
        _ => {
            println!("err");
            Vec::new()
        }
    };

    // let x: Vec<_> = data
    // .clone()
    // .into_iter()
    // .map(|i| {
    // let serialized = serde_json::to_string(&i).unwrap();
    // println!("serialized = {}", serialized);
    // serialized
    // })
    // .collect();
    // println!("{:?}", x);
    Ok(data)
}
