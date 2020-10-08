use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

const GITHUB_URL: &str = "https://github.com/trending/developers";

#[derive(Debug)]
pub struct Repo {
    name: Option<String>,
    description: Option<String>,
    url: Option<String>,
}

#[derive(Debug)]
pub struct Developer {
    name: Option<String>,
    username: Option<String>,
    url: Option<String>,
    avatar: Option<String>,
    repo: Option<Repo>,
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

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

        println!("x: {:?}", repo_url);
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
pub async fn developers() -> Result<Vec<Developer>, Box<dyn std::error::Error>> {
    let html = fetch_html(GITHUB_URL).await;
    let data: Vec<Developer> = match html {
        Ok(txt) => select_data(&txt),
        _ => {
            println!("err");
            Vec::new()
        }
    };
    println!("{:?}", data);
    Ok(data)
}
