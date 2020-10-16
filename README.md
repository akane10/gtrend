# gtrend

inspired by [github-trending-api](https://github.com/huchenme/github-trending-api).
scraping through [github/trending](https://github.com/trending).
for REST API see [gtrend-api](https://gitlab.com/akane10/gtrend-api)

## Installation

Cargo.toml

```
[dependencies]
gtrend = { git = "https://gitlab.com/akane10/gtrend", branch = "master" }
```

## Usage

```rust
use gtrend::Since::{Daily, Weekly, Monthly};
use gtrend::repos;
use gtrend::repos::Repository;
use gtrend::developers;
use gtrend::developers::Developer;
use std::error::Error;

fn main(){
    let repos_data: Result<Vec<Repository>, Box<Error>> = repos::builder()
          .programming_language("rust")
          .since(Daily)
          .spoken_language("en")
          .get_data().unwrap();

    let dev_data: Result<Vec<Developer>, Box<Error>> = developers::builder()
        .programming_language("rust")
        .since(Weekly)
        .get_data();

    let repos_data: serde_json::Value = repos::builder()
          .programming_language("rust")
          .since(Monthly)
          .spoken_language("en")
          .get_data_json();

    println!("repos_data {:?}", repos_data.unwrap());
    println!("dev_data {:?}", dev_data.unwrap());
    println!("repos_data_json {:?}", repos_data_json);
}
```

```rust
// repos struct
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

// developers struct
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
