pub mod developers;
pub mod repo;

fn main() {
    let data = repo::repo();
    developers::developers();
    // println!("{:?}", data);
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
