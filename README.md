# gtrend

inspired by [github-trending-api](https://github.com/huchenme/github-trending-api).
scraping through [github.com/trending](https://github.com/trending).
for REST API see [gtrend-api](https://gitlab.com/akane10/gtrend-api)

## Installation

Cargo.toml

```
[dependencies]
gtrend = { git = "https://gitlab.com/akane10/gtrend" }
```

## Usage

```rust
use gtrend::Since::{Daily, Weekly, Monthly};
use gtrend::{repos, developers, languages, Since, Language, spoken_languages};
use gtrend::repos::Repository;
use gtrend::developers::Developer;
use gtrend::Error;

fn main(){
    let repos_data: Result<Vec<Repository>, Error> = repos::builder()
          .programming_language("rust")
          .since(Daily)
          .spoken_language("en")
          .get_data();

    let dev_data: Result<Vec<Developer>, Error> = developers::builder()
          .programming_language("rust")
          .since(Weekly)
          .get_data();

    let repos_data: Result<serde_json::Value, Error> = repos::builder()
          .programming_language("rust")
          .since(Monthly)
          .spoken_language("en")
          .get_data_json();

    println!("repos_data {:?}", repos_data.unwrap());
    println!("dev_data {:?}", dev_data.unwrap());
    println!("repos_data_json {:?}", repos_data_json.unwrap());


    // Convert Since to String and str
    let since_str: &str = Daily.to_str();
    let since_string: String = Weekly.to_string(); 
    let since_from_str: Option<Since> = Since.from_str("monthly");

    // Available spoken language and programming language
    let programming_lang_list: Vec<Language> = languages::get_data();
    let spoken_lang_list: Vec<Language> = spoken_languages::get_data();

    let programmin_lang_list_json: Value = languages::get_data_json();
    let spoken_lang_list_json: Value = spoken_languages::get_data_json();
}
```

## Struct

### Repository
```rust
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
```

### Developer
```rust
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
```

### Language
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Language {
    #[serde(alias = "urlParam")]
    pub url_param: String,
    pub name: String,
}
```
