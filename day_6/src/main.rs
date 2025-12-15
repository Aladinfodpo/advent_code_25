use std::env;
use std::fs;

enum Operation {
    Add,
    Multiply,
}

struct Problem {
    params : Vec<u64>,
    operation: Operation,
}

impl Problem {
    fn solve(self) -> u64 {
        match self.operation {
            Operation::Add => self.params.into_iter().fold(0, |res, p| res + p),
            Operation::Multiply => self.params.into_iter().fold(1, |res, p| res * p),
        }
    }
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn process_data_p1(mut data: String) -> Vec<Problem> {
    while data.contains("  ") {
        data = data.replace("  ", " ");
    }
    let lines = data.lines().collect::<Vec<&str>>();
    let mut params_list = vec![];

    for i in 0..lines.len()-1 {
        params_list.push(lines[i].split_whitespace().collect::<Vec<&str>>());
    }

    let os = lines[lines.len()-1].split_whitespace().collect::<Vec<&str>>();
    let n = os.len();

    let mut problems: Vec<Problem> = Vec::with_capacity(n);
    for i in 0..n {
        let mut params: Vec<u64> = vec![];
        for j in 0..params_list.len() {
            params.push(params_list[j][i].parse::<u64>().expect("Failed to parse param"));
        }
        let operation = match os[i] {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operation: {}", os[i]),
        };
        problems.push(Problem {params, operation });
    }
    problems
}

fn process_data_p2(data: String) -> Vec<Problem> {    
    let lines = data.lines().collect::<Vec<&str>>();
    let o_line = (**lines.last().unwrap()).to_string().replace("*", "+");

    let mut nb_space = o_line.split("+").map(|s| s.len()).collect::<Vec<usize>>();
    nb_space.remove(0);
    *nb_space.last_mut().unwrap() += 1;

    let os = lines[lines.len()-1].chars().collect::<Vec<char>>();
    let n = nb_space.len();
    
    let params_list = lines[0..lines.len()-1].iter().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut problems: Vec<Problem> = Vec::with_capacity(n);

    let mut index : usize = 0;

    for i in 0..n {
        let mut params: Vec<u64> = vec![];

        for i_num in 0..nb_space[i] {
            let mut param_str = String::new();
            for j in 0..params_list.len() {
                param_str.push(params_list[j][index + i_num]);
            }
            params.push(param_str.trim().parse::<u64>().expect("Failed to parse param"));
        }
        
        let operation = match os[index] {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => panic!("Unknown operation: {}", os[index]),
        };
        problems.push(Problem {params, operation });

        index += nb_space[i] + 1;
    }
    problems
}

fn solve_problems(problems: Vec<Problem>) -> u64 {
    problems.into_iter().fold(0, |res, p| res + p.solve())
}

fn main() {
    println!("P1: {}", solve_problems(process_data_p1(get_file())));
    println!("P2: {}", solve_problems(process_data_p2(get_file())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let string = String::from("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        assert_eq!(solve_problems(process_data_p1(string)), 4277556);
    }

    #[test]
    fn test_p2() {
        let string = String::from("123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ");
        assert_eq!(solve_problems(process_data_p2(string)), 3263827);
    }
}