pub mod developers;
pub mod languages;
pub mod repos;
pub mod spoken_languages;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const GITHUB_BASE_URL: &str = "https://github.com";
const GITHUB_TRENDING_URL: &str = "https://github.com/trending";

trait BuilderT {
    fn programming_language(self, lang: &str) -> Self;
    fn since(self, since: Since) -> Self;
}

// https://stackoverflow.com/questions/39150216/implementing-a-trait-for-multiple-types-at-once
#[macro_export]
macro_rules! impl_builder_T {
    (for $($t:ty),+) => {
        $(impl BuilderT for $t {
            fn programming_language(mut self, lang: &str) -> Self {
                let lang_: Option<Language> = languages::find(By::Both(lang));

                match lang_ {
                        Some(val) => {
                        self.pro_lang = Some(val.url_param);
                        self
                    }
                    _ => {
                        self.pro_lang = Some(lang.to_string());
                        self
                    }
                }
            }

            fn since(mut self, since: Since) -> Self {
                let s: String = since.to_string();
                self.since = Some(s);
                self
            }

        })*
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Language {
    #[serde(alias = "urlParam")]
    pub url_param: String,
    pub name: String,
}

pub enum By<'a> {
    Name(&'a str),
    UrlParam(&'a str),
    Both(&'a str),
}

impl Language {
    pub fn get_data(bytes: &[u8]) -> Vec<Language> {
        let data = String::from_utf8_lossy(bytes);
        let data_lang: Vec<Language> = serde_json::from_str(&data).unwrap();

        data_lang
    }

    pub fn get_data_json(data: Vec<Language>) -> Value {
        let x: Vec<Value> = data.into_iter().map(|x| json!(x)).collect();
        let data_json: Value = Value::Array(x);

        data_json
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Since {
    Daily,
    Weekly,
    Monthly,
}

impl Since {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
        }
    }

    pub fn to_string(&self) -> String {
        let daily = String::from("daily");
        let weekly = String::from("weekly");
        let monthly = String::from("monthly");

        match self {
            Self::Daily => daily,
            Self::Weekly => weekly,
            Self::Monthly => monthly,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "daily" => Self::Daily,
            "weekly" => Self::Weekly,
            "monthly" => Self::Monthly,
            _ => Self::Daily,
        }
    }
}

async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::developers;
    use crate::repos;
    use crate::Since;

    // const SINCE: Since = Since::Daily;

    #[tokio::test]
    async fn fetch_html_github_repo() {
        let github_url: &str = "https://github.com/trending";
        let html = fetch_html(github_url).await;
        assert!(html.is_ok());
    }

    #[tokio::test]
    async fn fetch_html_github_developers() {
        let github_url: &str = "https://github.com/trending/developers";
        let html = fetch_html(github_url).await;
        assert!(html.is_ok());
    }

    #[test]
    fn since_to_str() {
        let x: &str = Since::Daily.to_str();
        assert_eq!(x, "daily");
    }

    #[test]
    fn since_from_str() {
        let x: Since = Since::from_str("daily");
        // println!("Display Since: {:?}", x);
        assert_eq!(x, Since::Daily);
    }

    #[test]
    fn languages() {
        let data = languages::get_data();

        assert!(data.len() > 0);
    }

    #[test]
    fn languages_json() {
        let data = languages::get_data_json();

        assert!(data.is_array());
    }

    #[test]
    fn spoken_languages() {
        let data = spoken_languages::get_data();

        assert!(data.len() > 0);
    }

    #[test]
    fn spoken_languages_json() {
        let data = spoken_languages::get_data_json();

        assert!(data.is_array());
    }

    #[test]
    fn repo() {
        let data = repos::builder().since(Since::Weekly).get_data();
        assert!(data.is_ok())
    }

    #[test]
    fn repo_json() {
        let data: serde_json::Value = repos::builder().since(Since::Weekly).get_data_json();

        assert!(data.is_array());
    }

    #[test]
    fn repo_should_not_be_empty() {
        let data = repos::builder().spoken_language("en").get_data().unwrap();
        assert!(data.len() > 0)
    }

    #[test]
    fn repo_author_should_always_some() {
        let data = repos::builder().get_data().unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.author.is_some())
            .collect();

        assert_eq!(y.len(), data.len())
    }

    #[test]
    fn repo_with_lang() {
        let data = repos::builder().programming_language("rust").get_data();
        // assert!(data.is_ok())
        assert!(data.unwrap().len() > 0);
    }

    #[test]
    fn repo_with_lang_and_spoken_lang() {
        let data = repos::builder()
            .spoken_language("en")
            .programming_language("rust")
            .get_data()
            .unwrap();

        let x: Vec<repos::Repository> = data
            .clone()
            .into_iter()
            .filter(|x| x.programming_language.as_ref().unwrap() == "Rust")
            .collect();

        assert!(data.len() == x.len());
    }

    #[test]
    fn repo_with_lang_since_and_spoken_lang() {
        let data = repos::builder()
            .spoken_language("en")
            .since(Since::Daily)
            .programming_language("rust")
            .get_data();

        assert!(data.unwrap().len() > 0);
    }

    #[test]
    fn repo_with_unknown_lang() {
        let data = repos::builder()
            .spoken_language("en")
            .programming_language("wdawdaw")
            .get_data();

        assert!(data.is_ok());
    }

    #[test]
    fn repo_with_cpp_lang() {
        let data = repos::builder()
            .since(Since::Daily)
            .programming_language("C++")
            .get_data()
            .unwrap();

        let x: Vec<repos::Repository> = data
            .clone()
            .into_iter()
            .filter(|x| x.programming_language.as_ref().unwrap() == "C++")
            .collect();

        assert!(data.len() == x.len());
    }

    #[test]
    fn repo_with_empty_spoken_lang() {
        let data = repos::builder()
            .spoken_language("")
            .since(Since::Daily)
            .programming_language("rust")
            .get_data();

        assert!(data.is_ok())
    }

    #[test]
    fn developers() {
        let data = developers::builder().get_data().unwrap();

        // println!("{:?}", data);
        assert!(data.len() > 0);
    }

    #[test]
    fn developers_json() {
        let data = developers::builder().get_data_json();

        // println!("{:?}", data);
        assert!(data.is_array());
    }

    #[test]
    fn developers_should_not_be_empty() {
        let data = developers::builder()
            .since(Since::Monthly)
            .get_data()
            .unwrap();

        assert!(data.len() > 0);
    }

    #[test]
    fn developers_with_lang() {
        let data = developers::builder()
            .programming_language("rust")
            .get_data()
            .unwrap();

        assert!(data.len() > 0);
    }

    #[test]
    fn developers_with_unknown_lang() {
        let data = developers::builder()
            .programming_language("unknown")
            .get_data();

        assert!(data.unwrap().len() > 0);
    }

    #[test]
    fn developers_username_should_always_some() {
        let data = developers::builder()
            .programming_language("rust")
            .since(Since::Daily)
            .get_data()
            .unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.username.is_some())
            .collect();

        assert_eq!(y.len(), data.len());
    }

    #[test]
    fn developers_name_should_always_some() {
        let data = developers::builder()
            .programming_language("rust")
            .since(Since::Daily)
            .get_data()
            .unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.name.is_some())
            .collect();

        assert_eq!(y.len(), data.len());
    }

    #[test]
    fn developers_url_should_always_some() {
        let data = developers::builder()
            .programming_language("rust")
            .since(Since::Daily)
            .get_data()
            .unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.url.is_some())
            .collect();

        assert_eq!(y.len(), data.len());
    }

    #[test]
    fn developers_avatar_should_always_some() {
        let data = developers::builder()
            .programming_language("rust")
            .since(Since::Daily)
            .get_data()
            .unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.avatar.is_some())
            .collect();

        assert_eq!(y.len(), data.len());
    }
}
