pub mod developers;
pub mod repo;

#[cfg(test)]
mod tests {
    use crate::developers;
    use crate::repo;

    #[test]
    fn get_repo() {
        let data = repo::repo();

        assert!(data.is_ok())
    }

    #[test]
    fn get_developers() {
        let data = developers::developers();

        assert!(data.is_ok());
    }
}
