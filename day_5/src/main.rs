use std::env;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }

    fn merge(&self, other: &Range) -> Option<Range> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(Range::new(
                self.start.min(other.start),
                self.end.max(other.end),
            ))
        }
    }

    fn cmp(&self, other: &Range) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }

}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn is_fresh(number : u64, list : &Vec<(u64, u64)>) -> bool {
    for i in 0..list.len() {
        if number >= list[i].0 && number <= list[i].1 {
            return true;
        }
    }

    false
}

fn number_fresh(fruits: Vec<u64>, list : &Vec<(u64, u64)>) -> u64 {
    let mut count = 0;
    for fruit in fruits {
        if is_fresh(fruit, list) {
            count += 1;
        }
    }

    count
}

fn number_range_sorted(list : Vec<Range>) -> u64 {
    let mut merged_dates: Vec<Range> = vec![list[0]];
    for date in list {
        match merged_dates.last().unwrap().merge(&date) {
            None => merged_dates.push(date),
            Some(merged_date) => {
                merged_dates.pop();
                merged_dates.push(merged_date);
            }
        }
    }

    merged_dates.iter().fold(0, |acc, date| acc + (date.end - date.start + 1))
}

fn check_file() -> u64{
    let file = get_file();
    let mut fruits_date = file.split("\r\n\r\n");
    let dates: Vec<(u64, u64)> = fruits_date.next().unwrap().lines().map(
        |x| {
            let mut date = x.split("-");
            (date.next().unwrap().parse().expect("Error parsing"), date.next().unwrap().parse().expect("Error parsing"))
        } 
    ).collect(); 
    let fruits: Vec<u64> = fruits_date.next().unwrap().lines().map(|x| x.parse().expect("Error parsing") ).collect();

    number_fresh(fruits, &dates)
}

fn number_range_string(string : String) -> u64{
    let mut fruits_date = string.split("\r\n\r\n");
    let mut dates: Vec<Range> = fruits_date.next().unwrap().lines().map(
        |x| {
            let mut date = x.split("-");
            Range::new(date.next().unwrap().parse().expect("Error parsing"), date.next().unwrap().parse().expect("Error parsing"))
        } 
    ).collect(); 

    dates.sort_by(|a, b| a.cmp(b));

    
    number_range_sorted(dates)
}

fn main() {
    println!("P1 : {}",check_file());
    println!("P2 : {}",number_range_string(get_file()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple() {
        let list = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        assert_eq!(is_fresh(1, &list), false);
        assert_eq!(is_fresh(5, &list), true);
        assert_eq!(is_fresh(8, &list), false);
        assert_eq!(is_fresh(11, &list), true);
        assert_eq!(is_fresh(17, &list), true);
        assert_eq!(is_fresh(32, &list), false);
    }

    #[test]
    fn test_exemple_p2() {
        let string = "3-5\n10-14\n16-20\n12-18".to_string();
        assert_eq!(number_range_string(string), 14);
    }
}