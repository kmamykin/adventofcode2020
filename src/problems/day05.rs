use super::super::utils::read_strings_from_file;
pub fn solve() {
    let strings = read_strings_from_file("./inputs/day05_1").expect("Failed to read inputs");
    println!("{:?}", strings);
}
