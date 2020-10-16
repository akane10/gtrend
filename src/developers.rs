use crate::{fetch_html, Since, GITHUB_BASE_URL, GITHUB_TRENDING_URL};
use select::document::Document;
use select::predicate::{Class, Name};
use serde::{Deserialize, Serialize};

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

impl Developer {
    pub fn json_stringify(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

pub struct Builder {
    pro_lang: Option<String>,
    since: Option<String>,
}

impl Builder {
    pub fn programming_language(mut self, lang: &'static str) -> Self {
        self.pro_lang = Some(lang.to_string());
        self
    }

    pub fn since(mut self, since: Since) -> Self {
        let s: String = since.to_string();
        self.since = Some(s);
        self
    }

    #[tokio::main]
    pub async fn get_data(self) -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
        let params_url: String = match (self.pro_lang, self.since) {
            (Some(l), Some(s)) => format!("/{}?since={}", l, s),
            (None, Some(s)) => format!("?since={}", s),
            (Some(l), None) => format!("/{}", l),
            _ => "".to_string(),
        };

        let url = format!("{}{}{}", GITHUB_TRENDING_URL, "/developers", params_url);

        // println!("{}", url);

        let html = fetch_html(&url).await;
        let data: Vec<Developer> = match html {
            Ok(txt) => select_data(&txt),
            _ => {
                println!("err");
                Vec::new()
            }
        };

        // println!("data result {:?}", data);
        Ok(data)
    }
}

fn select_data(html: &str) -> Vec<Developer> {
    let document = Document::from(html);

    let data: Vec<Developer> = document
        .find(Class("Box-row"))
        .into_iter()
        .map(|node| {
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

            let url: Option<String> = username
                .clone()
                .map(|x| format!("{}/{}", GITHUB_BASE_URL, x));

            let sponsor_url: Option<String> = node
                .find(Class("mr-2"))
                .next()
                .and_then(|x| x.find(Name("a")).next())
                .and_then(|x| x.attr("href"))
                .map(|x| format!("{}{}", GITHUB_BASE_URL, x));

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

            return Developer {
                name: name,
                username: username,
                url: url,
                sponsor_url: sponsor_url,
                avatar: avatar,
                repo: repo,
            };
        })
        .collect();

    data
}

pub fn builder() -> Builder {
    Builder {
        pro_lang: None,
        since: None,
    }
}
