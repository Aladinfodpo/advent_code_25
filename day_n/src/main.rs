use std::env;
use std::fs;

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    println!("Checking file...");
    println!("Results: {}", get_file());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(2, 2);
    }
}