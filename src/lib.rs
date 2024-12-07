use std::io::BufReader;

pub mod info;

pub fn solve_normal() -> Vec<String> {
    let input = BufReader::new(include_str!("input.txt").as_bytes());

    vec!["Answer".to_string(), input.get_ref().len().to_string()]
}
