# gtrend
inspired by [github-trending-api](https://github.com/huchenme/github-trending-api).
scraping through [github/trending](https://github.com/trending).
for api see [gtrend-api](https://gitlab.com/akane10/gtrend-api)

## Installation
Cargo.toml
```
[dependencies]
gtrend = { git = "https://gitlab.com/akane10/gtrend", branch = "master" }
```

## Usage
```rust
use crate::developers;
use crate::gtrend::Since;
use crate::repos;

const SINCE: Since = Since::Daily;

fn main(){
    let programming_language = String::new("rust");
    let spoken_language = String::new("en");
    
    let repos_data = repos::get_data(
                        Some(programming_language), // or None
                        SINCE, 
                        Some(spoken_language) // or None
                    );
    
    let dev_data = developers::get_data(Some(programming_language), SINCE);
    
    println!("repos_data {:?}", repos_data.unwrap());
    println!("dev_data {:?}", dev_data.unwrap());
    
    let repos_data_json: Vec<_> = repos_data.unwrap()
       .into_iter()
       .map(|repo| {
           let serialized = serde_json::to_string(&repo).unwrap();
           // println!("serialized = {}", serialized);
           serialized
       })
       .collect();

    println!("repos_data_json {:?}", repos_data_json);
}
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
    pub current_star: Option<i32>,
    pub description: Option<String>,
    pub programming_language: Option<String>,
    pub url: Option<String>,
    pub stars: Option<i32>,
    pub forks: Option<i32>,
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
