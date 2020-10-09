pub mod developers;
pub mod gtrend;
pub mod helpers;
pub mod repo;
// use crate::helpers;

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
}
