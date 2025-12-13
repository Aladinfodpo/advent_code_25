use std::env;
use std::fs;

fn rec_check_num_str(mut remain : String, mut buf : String, is_shrinking: bool) -> bool{
    if !remain.is_empty() {
        let digit = remain.remove(0);
        let mut res = false;

        if buf.chars().nth(0) == Some(digit) {
            let mut new_buf = buf.clone();
            new_buf.remove(0);
            res = res | rec_check_num_str(remain.clone(), new_buf, true);
        }

        if !is_shrinking {
            buf.push(digit);
            res |= rec_check_num_str(remain, buf, false);
        }
        res
    }
    else {
        buf.is_empty()
    }
}

fn check_number(number: u64) -> bool{
    rec_check_num_str(number.to_string(), String::new(), false)
}


fn check_range(range : (u64, u64)) -> u64{
    (range.0..=range.1).fold(0, |res, number|{res + if check_number(number) {1} else {0} })
}

fn sum_check_range(range : (u64, u64)) -> u64{
    (range.0..=range.1).fold(0, |res, number|{res + if check_number(number) {number} else {0} })
}

fn check_file() -> u64{
    let filename = get_file_path("input.txt");
    let contents = fs::read_to_string(filename).expect("File is missing");
    contents.split(',').fold(0, |res, str|{ res + sum_check_range(range_from_str(str)) })
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
    println!("Sum of corrupt ID : {}", check_file());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(check_range((11, 22)), 2);
        assert_eq!(check_range((1188511880,1188511890)), 1);
        assert_eq!(check_range((222220, 222224)), 1);
        assert_eq!(check_range((1698522, 1698528)), 0);
        assert_eq!(check_range((446443, 446449)), 1);
        assert_eq!(check_range((38593856, 38593862)), 1);
        assert_eq!(check_range((2121212118, 2121212124)), 0);
        assert_eq!(check_range((565653, 565659)), 0);
    }

    #[test]
    fn test_str() {
        assert_eq!(check_range(range_from_str("11-22")), 2);
        assert_eq!(check_range(range_from_str("1188511880-1188511890")), 1);
        assert_eq!(check_range(range_from_str("222220-222224")), 1);
        assert_eq!(check_range(range_from_str("1698522-1698528")), 0);
        assert_eq!(check_range(range_from_str("446443-446449")), 1);
        assert_eq!(check_range(range_from_str("38593856-38593862")), 1);
        assert_eq!(check_range(range_from_str("2121212118-2121212124")), 0);
        assert_eq!(check_range(range_from_str("565653-565659")), 0);
    }

     #[test]
    fn test_number() {
        assert_eq!(check_number(11), true);
        assert_eq!(check_number(110110), true);
        assert_eq!(check_number(2121212121), false);
        assert_eq!(check_number(123123123), false);

        assert_eq!(check_number(1212), true);
    }

    #[test]
    fn test_parsing(){
        assert_eq!("9393974421".parse(), Ok(9393974421u64));
        assert_eq!("9393862801".parse(), Ok(9393862801u64));
    }
}