use std::env;
use std::fs;


fn find_max_in_string_p1(s: &str) -> u64 {
    find_max_in_string_p2(s, 2)
}

fn find_max_in_string_p2(s: &str, mut nb : usize) -> u64 {
    let numbers = s.chars().map(|c| c.to_digit(10).expect(format!("Error parsing char : {}", c).as_str()) as u64).collect::<Vec<u64>>();
    let n = numbers.len(); 

    let mut last_index = 0;
    let mut res = 0u64;

    while nb > 0 {
        let mut slice_s = numbers[last_index..n-nb+1].iter();
        let digit = *slice_s.clone().max().expect("Empty list ?");
        last_index += slice_s.position(|x| *x == digit).unwrap() + 1;

        nb -= 1;
        res = res * 10 + digit;
    }
    res
}

fn get_joltage_file_p1() -> u64{
    get_file().lines().fold(0u64, |acc, line| acc + find_max_in_string_p1(line))
}

fn get_joltage_file_p2() -> u64{
    get_file().lines().fold(0u64, |acc, line| acc + find_max_in_string_p2(line, 12))
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    println!("P1 : {}", get_joltage_file_p1());
    println!("P2 : {}", get_joltage_file_p2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple() {
        assert_eq!(find_max_in_string_p1("987654321111111"), 98);
        assert_eq!(find_max_in_string_p1("811111111111119"), 89);
        assert_eq!(find_max_in_string_p1("234234234234278"), 78);
        assert_eq!(find_max_in_string_p1("818181911112111"), 92);
    }

    #[test]
    fn test_exemple_p2() {
        assert_eq!(find_max_in_string_p2("987654321111111", 12), 987654321111u64);
        assert_eq!(find_max_in_string_p2("811111111111119", 12), 811111111119u64);
        assert_eq!(find_max_in_string_p2("234234234234278", 12), 434234234278u64);
        assert_eq!(find_max_in_string_p2("818181911112111", 12), 888911112111u64);
    }

}