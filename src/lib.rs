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
        let since = repo::Since::Daily;
        let data = repo::get_data(None, since);

        assert!(data.is_ok())
    }

    #[test]
    fn get_repo_with_lang() {
        let since = repo::Since::Daily;
        let data = repo::get_data(Some("javascript"), since);

        assert!(data.is_ok())
    }

    #[test]
    fn get_developers() {
        let data = developers::get_data();

        assert!(data.is_ok());
    }
}
