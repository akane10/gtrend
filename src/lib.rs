pub mod developers;
pub mod gtrend;
pub mod helpers;
pub mod repo;

#[cfg(test)]
mod tests {
    use crate::developers;
    use crate::gtrend::Since;
    use crate::repo;

    const SINCE: Since = Since::Daily;
    #[test]
    fn get_repo() {
        let data = repo::get_data(None, SINCE, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_author_should_always_some() {
        let data = repo::get_data(None, SINCE, None).unwrap();

        let y: Vec<_> = data
            .clone()
            .into_iter()
            .filter(|x| x.author.is_none())
            .collect();

        assert!(y.is_empty())
    }

    #[test]
    fn get_repo_with_lang() {
        let data = repo::get_data(Some("rust"), SINCE, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_unknown_lang() {
        let data = repo::get_data(Some("unknown"), SINCE, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_developers() {
        let data = developers::get_data(None, SINCE);

        assert!(data.is_ok());
    }

    #[test]
    fn get_developers_with_lang() {
        let data = developers::get_data(Some("rust"), SINCE);

        assert!(data.is_ok());
    }

    #[test]
    fn get_developers_with_unknown_lang() {
        let data = developers::get_data(Some("unknown"), SINCE);

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
