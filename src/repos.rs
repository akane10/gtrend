use crate::*;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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

impl Repository {
    pub fn json_stringify(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[derive(Debug, Clone)]
pub struct Builder {
    pro_lang: Option<String>,
    spoken_lang: Option<String>,
    since: Option<String>,
}

impl_builder_T!(for Builder);

impl Builder {
    pub fn spoken_language(mut self, s_lang: &str) -> Self {
        let s_lang_: Option<Language> = spoken_languages::find(By::Both(s_lang));

        match s_lang_ {
            Some(val) => {
                self.spoken_lang = Some(val.url_param.clone());
                self
            }
            _ => {
                self.spoken_lang = Some(s_lang.to_string());
                self
            }
        }
    }

    pub async fn get_data_json(&self) -> Value {
        let data = self.get_data().await.unwrap_or(Vec::new());
        let data_json: Vec<Value> = data.into_iter().map(|x| json!(x)).collect();

        Value::Array(data_json)
    }

    pub async fn get_data(&self) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
        let pro_lang_url: String = self
            .pro_lang
            .as_ref()
            .map(|x| format!("/{}", x))
            .unwrap_or("".to_string());

        let optional_params: String = match (self.since.as_ref(), self.spoken_lang.as_ref()) {
            (Some(s), Some(sl)) => format!("?since={}&spoken_language_code={}", s, sl),
            (Some(s), None) => format!("?since={}", s),
            (None, Some(sl)) => format!("?spoken_language_code={}", sl),
            _ => "".to_string(),
        };

        let url = format!("{}{}{}", GITHUB_TRENDING_URL, pro_lang_url, optional_params);

        // println!("{}", url);

        let html = fetch_html(&url).await;
        let data: Vec<Repository> = match html {
            Ok(txt) => select_data(&txt),
            Err(e) => {
                println!("err: {}", e);
                Vec::new()
            }
        };

        // println!("data result {:?}", data);
        Ok(data)
    }
}

fn select_data(html: &str) -> Vec<Repository> {
    let document = Document::from(html);

    let data: Vec<Repository> = document
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

            let (username, reponame) = node
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
                })
                .unwrap_or((None, None));

            let current_star: Option<u32> =
                node.find(Class("float-sm-right")).next().and_then(|tag| {
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

            let url: Option<String> = match (username.clone(), reponame.clone()) {
                (Some(username), Some(reponame)) => {
                    Some(format!("{}/{}/{}", GITHUB_BASE_URL, username, reponame))
                }
                _ => None,
            };

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
                    let href = username
                        .clone()
                        .map(|x| format!("{}/{}", GITHUB_BASE_URL, x));

                    let built_by = BuiltBy {
                        username: username,
                        avatar: avatar,
                        href: href,
                    };

                    built_by
                })
                .collect::<Vec<_>>();

            // println!("x: {:?}", build_by);
            return Repository {
                avatar: username
                    .clone()
                    .map(|x| format!("{}/{}.png", GITHUB_BASE_URL, x)),
                author: username.clone(),
                name: reponame.clone(),
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
        })
        .collect();

    data
}

pub fn builder() -> Builder {
    Builder {
        pro_lang: None,
        spoken_lang: None,
        since: None,
    }
}
