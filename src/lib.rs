pub mod developers;
pub mod repos;

const GITHUB_BASE_URL: &str = "https://github.com";
const GITHUB_TRENDING_URL: &str = "https://github.com/trending";

pub async fn fetch_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}

#[derive(PartialEq, Debug)]
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

    pub fn from_str(s: &str) -> Self {
        match s {
            "daily" => Self::Daily,
            "weekly" => Self::Weekly,
            "monthly" => Self::Monthly,
            _ => Self::Daily,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::developers;
    use crate::repos;
    use crate::Since;

    #[tokio::test]
    async fn test_fetch_html_github_repo() {
        let github_url: &str = "https://github.com/trending";
        let html = fetch_html(github_url).await;
        assert!(html.is_ok());
    }

    #[tokio::test]
    async fn test_fetch_html_github_developers() {
        let github_url: &str = "https://github.com/trending/developers";
        let html = fetch_html(github_url).await;
        assert!(html.is_ok());
    }

    #[test]
    fn test_since_to_string() {
        let x: &str = Since::Daily.to_str();
        assert_eq!(x, "daily");
    }

    #[test]
    fn test_since_from_string() {
        let x: Since = Since::from_str("daily");
        println!("Display Since: {:?}", x);
        assert_eq!(x, Since::Daily);
    }

    const SINCE: Since = Since::Daily;
    #[test]
    fn get_repo() {
        let data = repos::get_data(None, SINCE, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_author_should_always_some() {
        let data = repos::get_data(None, SINCE, None).unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.author.is_none())
            .collect();

        assert!(y.is_empty())
    }

    #[test]
    fn get_repo_with_lang() {
        let data = repos::get_data(Some("rust".to_string()), SINCE, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_lang_and_spoken_lang() {
        let data = repos::get_data(Some("haskell".to_string()), SINCE, Some("en".to_string()));

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_unknown_lang() {
        let data = repos::get_data(Some("unknown".to_string()), SINCE, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_developers() {
        let data = developers::get_data(None, SINCE);

        assert!(data.is_ok());
    }

    #[test]
    fn get_developers_with_lang() {
        let data = developers::get_data(Some("rust".to_string()), SINCE);

        assert!(data.is_ok());
    }

    #[test]
    fn get_developers_with_unknown_lang() {
        let data = developers::get_data(Some("unknown".to_string()), SINCE);

        assert!(data.is_ok())
    }

    #[test]
    fn get_developers_username_should_always_some() {
        let data = developers::get_data(None, SINCE).unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.username.is_none())
            .collect();

        assert!(y.is_empty())
    }

    #[test]
    fn get_developers_name_should_always_some() {
        let data = developers::get_data(None, SINCE).unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.name.is_none())
            .collect();

        assert!(y.is_empty())
    }

    #[test]
    fn get_developers_url_should_always_some() {
        let data = developers::get_data(None, SINCE).unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.url.is_none())
            .collect();

        assert!(y.is_empty())
    }

    #[test]
    fn get_developers_avatar_should_always_some() {
        let data = developers::get_data(None, SINCE).unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.avatar.is_none())
            .collect();

        assert!(y.is_empty())
    }
}
