pub mod repo;

fn main() {
    let data = repo::repo();

    println!("{:?}", data);
}

// #[cfg(test)]
// mod tests {
// use crate::repo;

// #[test]
// fn get_repo() {
// let data = repo::repo();

// assert_eq!()
// }
// }
