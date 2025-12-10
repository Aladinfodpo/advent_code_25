use std::env;
use std::fs;
use std::u32;

#[derive(Clone)]
struct MatchingData {
    pattern : String,
    index : usize,
    count : u32,
    count_max : u32,
    state : ParsingState,
}

impl MatchingData {
    fn new(max : u32) -> MatchingData {
        MatchingData {
            pattern : String::new(),
            index : 0,
            count : 0,
            count_max : max,
            state : ParsingState::Creating,
        }
    }
}

#[derive(Clone)]
enum ParsingState {
    Creating,
    Matching,
}

fn rec_check_num_str(mut remain : String, mut data : MatchingData) -> bool{
    if !remain.is_empty() {
        let digit = remain.remove(0);
        let mut res = false;

        if data.pattern.chars().nth(data.index) == Some(digit) {
            let mut new_data = data.clone();
            new_data.state = ParsingState::Matching;
            new_data.index += 1;

            if new_data.index == new_data.pattern.len() {
                new_data.index = 0;
                new_data.count += 1;
            }

            res = res | rec_check_num_str(remain.clone(), new_data);
        }

        if matches!(data.state, ParsingState::Creating) {
            data.pattern.push(digit);
            res |= rec_check_num_str(remain, data);
        }
        res
    }
    else {
        matches!(data.state, ParsingState::Matching) && data.index == 0 && data.count < data.count_max && data.count > 0
    }
}

fn check_number(number: u64, data : MatchingData) -> bool{
    rec_check_num_str(number.to_string(), data)
}

#[allow(dead_code)]
fn check_number_p1(number: u64) -> bool{
    rec_check_num_str(number.to_string(), MatchingData::new(2))
}

#[allow(dead_code)]
fn check_number_p2(number: u64) -> bool{
    rec_check_num_str(number.to_string(), MatchingData::new(u32::MAX))
}

#[allow(dead_code)]
fn check_range_p1(range : (u64, u64)) -> u64{
    sum_check_range(range, MatchingData::new(2), false)
}

#[allow(dead_code)]
fn check_range_p2(range : (u64, u64)) -> u64{
    sum_check_range(range, MatchingData::new(u32::MAX), false)
}

fn sum_range_p1(range : (u64, u64)) -> u64{
    sum_check_range(range, MatchingData::new(2), true)
}

fn sum_range_p2(range : (u64, u64)) -> u64{
    sum_check_range(range, MatchingData::new(u32::MAX), true)
}

fn sum_check_range(range : (u64, u64), data : MatchingData, sum : bool) -> u64{
    (range.0..=range.1).fold(0, |res, number|{res + if check_number(number, data.clone()) {if sum {number} else {1}} else {0} })
}

fn check_file() -> (u64, u64){
    let filename = get_file_path("input.txt");
    let contents = fs::read_to_string(filename).expect("File is missing");
    (contents.split(',').fold(0, |res, str|{ res + sum_range_p1(range_from_str(str)) }),
    contents.split(',').fold(0, |res, str|{ res + sum_range_p2(range_from_str(str)) }))
}

fn range_from_str(string: &str) -> (u64, u64){
    let pair : Vec<&str> = string.split('-').collect();
    if pair.len() < 2 { panic!("Unrecognized range");}
    println!("{} to {}", pair[0], pair[1]);
    (pair[0].parse().expect("Unrecognized start"), pair[1].parse().expect("Unrecognized end"))
}


fn get_file_path(file: &str) -> std::path::PathBuf {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    exe_path.parent().unwrap().join(file)
}

fn main() {
    let (part1, part2) = check_file();
    println!("Sum of corrupt ID : P1 = {}, P2 = {}", part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(check_range_p1((11, 22)), 2);
        assert_eq!(check_range_p1((1188511880,1188511890)), 1);
        assert_eq!(check_range_p1((222220, 222224)), 1);
        assert_eq!(check_range_p1((1698522, 1698528)), 0);
        assert_eq!(check_range_p1((446443, 446449)), 1);
        assert_eq!(check_range_p1((38593856, 38593862)), 1);
        assert_eq!(check_range_p1((2121212118, 2121212124)), 0);
        assert_eq!(check_range_p1((565653, 565659)), 0);
    }

    #[test]
    fn test_range_p2() {
        assert_eq!(check_range_p2((11, 22)), 2);
        assert_eq!(check_range_p2((95, 115)), 2);
        assert_eq!(check_range_p2((1188511880,1188511890)), 1);
        assert_eq!(check_range_p2((222220, 222224)), 1);
        assert_eq!(check_range_p2((1698522, 1698528)), 0);
        assert_eq!(check_range_p2((446443, 446449)), 1);
        assert_eq!(check_range_p2((38593856, 38593862)), 1);
        assert_eq!(check_range_p2((2121212118, 2121212124)), 1);
        assert_eq!(check_range_p2((565653, 565659)), 1);
    }

    #[test]
    fn test_str() {
        assert_eq!(check_range_p1(range_from_str("11-22")), 2);
        assert_eq!(check_range_p1(range_from_str("1188511880-1188511890")), 1);
        assert_eq!(check_range_p1(range_from_str("222220-222224")), 1);
        assert_eq!(check_range_p1(range_from_str("1698522-1698528")), 0);
        assert_eq!(check_range_p1(range_from_str("446443-446449")), 1);
        assert_eq!(check_range_p1(range_from_str("38593856-38593862")), 1);
        assert_eq!(check_range_p1(range_from_str("2121212118-2121212124")), 0);
        assert_eq!(check_range_p1(range_from_str("565653-565659")), 0);
    }

     #[test]
    fn test_number_p1() {
        assert_eq!(check_number_p1(11), true);
        assert_eq!(check_number_p1(110110), true);
        assert_eq!(check_number_p1(2121212121), false);
        assert_eq!(check_number_p1(123123123), false);

        assert_eq!(check_number_p1(1212), true);
        assert_eq!(check_number_p1(11111), false);
    }

     #[test]
    fn test_number_p2() {
        assert_eq!(check_number_p2(11), true);
        assert_eq!(check_number_p2(110110), true);
        assert_eq!(check_number_p2(2121212121), true);
        assert_eq!(check_number_p2(123123123), true);

        assert_eq!(check_number_p2(1212), true);
        assert_eq!(check_number_p2(11111), true);
    }

    #[test]
    fn test_parsing(){
        assert_eq!("9393974421".parse(), Ok(9393974421u64));
        assert_eq!("9393862801".parse(), Ok(9393862801u64));
    }
}