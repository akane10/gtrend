pub mod developers;
pub mod helpers;
pub mod repo;
// use crate::helpers;

#[cfg(test)]
mod tests {
    use crate::developers;
    use crate::repo;

    #[test]
    fn get_repo() {
        let data = repo::get_data(None, None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_lang() {
        let data = repo::get_data(Some("javascript"), None);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_since() {
        let data = repo::get_data(None, Some("weekly"));

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_lang_and_since() {
        let data = repo::get_data(Some("rust"), Some("weekly"));

        assert!(data.is_ok())
    }

    #[test]
    fn get_developers() {
        let data = developers::get_data();

        assert!(data.is_ok());
    }
}
