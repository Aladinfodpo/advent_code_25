use std::env;
use std::fs;

static HAVE_ROLL: char = '@';
static NO_ROLL: char = '.';

fn count_accessible(lines: Vec<String>) -> u32{
    let nx = lines[0].len() as i32;
    let ny = lines.len() as i32;

    let mut res = 0;

    for x in 0..nx {
        for y in 0..ny {
            if lines[y as usize].chars().nth(x as usize).unwrap() != HAVE_ROLL {
                continue;
            }

            let mut neighbors = 0;
            for dx in -1..=1{
                if x + dx >= 0 && x + dx < nx {
                    for dy in -1..=1{
                        if y + dy >= 0 && y + dy < ny {
                            let case = lines[(y+dy) as usize].chars().nth((x+dx) as usize).unwrap(); 
                            if case == HAVE_ROLL && !(dx == 0 && dy == 0) {
                                neighbors += 1;
                            }
                        }
                    }
                }
            }
            if neighbors < 4 {
                res += 1;
            }

        }
    }

    res
}

fn remove_accessible(lines: Vec<String>) -> Vec<String>{
    let nx = lines[0].len() as i32;
    let ny = lines.len() as i32;

    let mut res = Vec::new();
    
    for y in 0..ny {
        res.push(String::new());
        for x in 0..nx {
            if lines[y as usize].chars().nth(x as usize).unwrap() != HAVE_ROLL {
                res[y as usize].push(NO_ROLL);
                continue;
            }

            let mut neighbors = 0;
            for dx in -1..=1{
                if x + dx >= 0 && x + dx < nx {
                    for dy in -1..=1{
                        if y + dy >= 0 && y + dy < ny {
                            let case = lines[(y+dy) as usize].chars().nth((x+dx) as usize).unwrap(); 
                            if case == HAVE_ROLL && !(dx == 0 && dy == 0) {
                                neighbors += 1;
                            }
                        }
                    }
                }
            }

            res[y as usize].push(if neighbors < 4 {NO_ROLL} else {HAVE_ROLL});

        }
    }
    res

}

fn count_isolated(lines: Vec<String>) -> u32{
    let mut last_nb = count_roll(&lines);
    let mut current_lines = lines;

    last_nb - loop {
        current_lines = remove_accessible(current_lines);
        let current_nb = count_roll(&current_lines);
        if current_nb == last_nb { break current_nb}
        last_nb = current_nb;
    }
}

fn count_roll(lines: &Vec<String>) -> u32{
    let nx = lines[0].len() as i32;
    let ny = lines.len() as i32;

    let mut res = 0;

    for x in 0..nx {
        for y in 0..ny {
            if lines[y as usize].chars().nth(x as usize).unwrap() == HAVE_ROLL {
                res += 1;
            }
        }
    }
    res
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    println!("P1 : {}", count_accessible(get_file().lines().map(|s| s.to_string()).collect()));
    println!("P2 : {}", count_isolated(get_file().lines().map(|s| s.to_string()).collect()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple() {
        let string = "..@@.@@@@.\n\
                            @@@.@.@.@@\n\
                            @@@@@.@.@@\n\
                            @.@@@@..@.\n\
                            @@.@@@@.@@\n\
                            .@@@@@@@.@\n\
                            .@.@.@.@@@\n\
                            @.@@@.@@@@\n\
                            .@@@@@@@@.\n\
                            @.@.@@@.@.";
        assert_eq!(count_accessible(string.replace(" ", "").as_str().lines().map(|s| s.to_string()).collect()), 13);
    }

    #[test]
    fn test_exemple_p2() {
        let string = "..@@.@@@@.\n\
                            @@@.@.@.@@\n\
                            @@@@@.@.@@\n\
                            @.@@@@..@.\n\
                            @@.@@@@.@@\n\
                            .@@@@@@@.@\n\
                            .@.@.@.@@@\n\
                            @.@@@.@@@@\n\
                            .@@@@@@@@.\n\
                            @.@.@@@.@.";
        assert_eq!(count_isolated(string.replace(" ", "").as_str().lines().map(|s| s.to_string()).collect()), 43);
    }
}