use select::document::Document;
use select::predicate::{Attr, Class, Name};

use crate::gtrend::Since;
// use crate::gtrend::Since::*;
use crate::helpers;
use serde::{Deserialize, Serialize};

const GITHUB_BASE: &str = "https://github.com";
const GITHUB_URL: &str = "https://github.com/trending";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltBy {
    pub username: Option<String>,
    pub href: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub avatar: Option<String>,
    pub author: Option<String>,
    pub name: Option<String>,
    pub current_star: Option<u32>,
    pub description: Option<String>,
    pub programming_language: Option<String>,
    pub url: Option<String>,
    pub stars: Option<u32>,
    pub forks: Option<u32>,
    pub lang_color: Option<String>,
    pub built_by: Vec<BuiltBy>,
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

        let username_reponame: Option<(Option<String>, Option<String>)> = node
            .find(Name("h1"))
            .next()
            .and_then(|x| x.find(Name("a")).next())
            .and_then(|x| x.attr("href"))
            .map(|x| {
                let y = x.split("/").collect::<Vec<_>>();
                let username = match y.len() {
                    n if n > 1 => Some(y[1].to_string()),
                    _ => None,
                };
                let reponame = match y.len() {
                    n if n > 2 => Some(y[2].to_string()),
                    _ => None,
                };
                (username, reponame)
            });

        let current_star: Option<u32> = node.find(Class("float-sm-right")).next().and_then(|tag| {
            let x = tag.text();
            let s: Vec<_> = x.split_whitespace().collect();
            Some(s[0].replace(",", "").parse::<u32>().unwrap())
        });

        let lang: Option<String> = node
            .find(Attr("itemprop", "programmingLanguage"))
            .next()
            .and_then(|x| Some(x.text()));

        let desc: Option<String> = node
            .find(Name("p"))
            .next()
            .and_then(|x| Some(escape(x.text())));

        let url: Option<String> = username_reponame.clone().and_then(|x| {
            let github = String::from(GITHUB_BASE);
            let str_ = match (x.0, x.1) {
                (Some(u), Some(r)) => Some(format!("{}/{}/{}", github, u, r)),
                _ => None,
            };
            str_
        });

        let stars_forks: Vec<u32> = node
            .find(Class("muted-link"))
            .map(|x| {
                let s = escape(x.text()).replace(",", "");
                s.parse::<u32>().unwrap()
            })
            .collect::<Vec<_>>();

        let lang_color: Option<String> = node
            .find(Class("repo-language-color"))
            .next()
            .and_then(|x| x.attr("style"))
            .map(|x| x.replace("background-color: ", ""));

        let built_by: Vec<BuiltBy> = node
            .find(Class("avatar-user"))
            .map(|x| {
                let username: Option<String> = x.attr("alt").and_then(|val| {
                    let u: Vec<&str> = val.split("@").collect();

                    if u.len() > 1 {
                        Some(u[1].to_string())
                    } else {
                        None
                    }
                });
                let avatar = x.attr("src").and_then(|a| {
                    let ss: Vec<&str> = a.split("?").collect();

                    if ss.len() > 0 {
                        Some(ss[0].to_string())
                    } else {
                        None
                    }
                });
                let href = username.clone().map(|x| format!("{}/{}", GITHUB_BASE, x));

                let built_by = BuiltBy {
                    username: username,
                    avatar: avatar,
                    href: href,
                };

                built_by
            })
            .collect::<Vec<_>>();

        // println!("x: {:?}", build_by);
        let repo: Repository = Repository {
            avatar: username_reponame.clone().and_then(|x| match x.0 {
                Some(val) => Some(format!("{}/{}.png", GITHUB_BASE, val)),
                _ => None,
            }),
            author: username_reponame.clone().and_then(|x| x.0),
            name: username_reponame.clone().and_then(|x| x.1),
            current_star: current_star,
            programming_language: lang,
            description: desc,
            url: url,
            stars: match stars_forks.len() {
                n if n > 0 => Some(stars_forks[0].clone()),
                _ => None,
            },
            forks: match stars_forks.len() {
                n if n > 1 => Some(stars_forks[1].clone()),
                _ => None,
            },
            built_by: built_by,
            lang_color: lang_color,
        };
        vec.push(repo);
    }
    vec
}

#[tokio::main]
pub async fn get_data(
    lang: Option<String>,
    since: Since,
    spoken_lang: Option<String>,
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

    // println!("{:?}", x);
    Ok(data)
}

/*
* to json example
*
     let x: Vec<_> = data
       .into_iter()
       .map(|i| {
           let serialized = serde_json::to_string(&i).unwrap();
           // println!("serialized = {}", serialized);
           serialized
       })
       .collect();
* */