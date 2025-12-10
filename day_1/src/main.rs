use std::fs;
use std::env;

static MAX_POSITION: u32 = 100;
struct Safe {
    position: u32,
}

struct Turn {
    left: bool,
    number: u32,
}

impl Turn {
    fn from_str(s: &str) -> Turn {
        let left = s.get(0..1) == Some("L");
        let number: u32 = s.get(1..).unwrap_or("0").parse().unwrap();
        Turn { left, number : number}
    }

    fn from_file(path: &str) -> Vec<Turn> {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        contents.lines().map(|line| Turn::from_str(line)).collect()
    }
}


impl Safe {
    fn turn(&mut self, turn: Turn) -> u32 {
        if turn.left {
            if self.position <= turn.number {
                let mut res = (turn.number - self.position) / MAX_POSITION + 1;
                if self.position == 0 {
                    res -= 1;
                    
                }
                self.position = MAX_POSITION - (turn.number - self.position) % MAX_POSITION;
                self.position = self.position % MAX_POSITION;
                res
                
            }else{
                self.position -= turn.number;
                0
            }
        } else {
            self.position += turn.number;
            let res = self.position / MAX_POSITION;
            self.position %= MAX_POSITION;
            res
        }
    }

    fn is_null(&self) -> bool {
        self.position == 0
    }

    fn apply_turns(&mut self, turns: Vec<Turn>) -> (u32, u32) {
        let mut nb_zeros_p2 : u32 = 0;
        let mut nb_zeros_p1 : u32 = 0;
        for turn in turns {
            nb_zeros_p2 += self.turn(turn);
            if self.is_null() { nb_zeros_p1 += 1; }
        }
        (nb_zeros_p1, nb_zeros_p2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_safe() {
        let mut safe = Safe { position: 50 };
        safe.turn(Turn { left: true, number: 68 });
        assert_eq!(safe.position, 82);  
        safe.turn(Turn { left: true, number: 30 });
        assert_eq!(safe.position, 52);
        safe.turn(Turn { left: false, number: 48 });
        assert_eq!(safe.position, 0);  
    }

    #[test]
    fn test_turn_from_str() {
        let turn1 = Turn::from_str("L68");
        assert_eq!(turn1.left, true);
        assert_eq!(turn1.number, 68);

        let turn1 = Turn::from_str("R208");
        assert_eq!(turn1.left, false);
        assert_eq!(turn1.number, 8);
    }

    #[test]
    fn test_apply_turns() {
        {
            let mut safe = Safe { position: 50 };
            let turns = vec![
                Turn::from_str("L68"),
                Turn::from_str("L30"),
                Turn::from_str("R48"),
                Turn::from_str("L5"),
                Turn::from_str("R60"),
                Turn::from_str("L55"),
                Turn::from_str("L1"),
                Turn::from_str("L99"),
                Turn::from_str("R14"),
                Turn::from_str("L82"),
                ];
            let nb_zeros = safe.apply_turns(turns);
            assert_eq!(nb_zeros, (3, 6));
        }
        {
            let mut safe = Safe { position: 99 };
            let turns = vec![
                Turn::from_str("R1000"),
                ];
            let nb_zeros = safe.apply_turns(turns);
            assert_eq!(nb_zeros, (0, 10));
        }
        {
            let mut safe = Safe { position: 0 };
            let turns = vec![
                Turn::from_str("R1000"),
                ];
            let nb_zeros = safe.apply_turns(turns);
            assert_eq!(nb_zeros, (1, 10));
        }
        {
            let mut safe = Safe { position: 99 };
            let turns = vec![
                Turn::from_str("L1000"),
                ];
            let nb_zeros = safe.apply_turns(turns);
            assert_eq!(nb_zeros, (0, 10));
        }
        {
            let mut safe = Safe { position: 0 };
            let turns = vec![
                Turn::from_str("L1000"),
                ];
            let nb_zeros = safe.apply_turns(turns);
            assert_eq!(nb_zeros, (1, 10));
        }
        {
            let mut safe = Safe { position: 0 };
            let turns = vec![
                Turn::from_str("L0"),
                ];
            let nb_zeros = safe.apply_turns(turns);
            assert_eq!(nb_zeros, (1, 0));
        }
    }

}

fn get_file_path(file: &str) -> std::path::PathBuf {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    exe_path.parent().unwrap().join(file)
}

fn main() { 
    let turns = Turn::from_file(get_file_path("input.txt").to_str().unwrap());
    let mut safe = Safe { position: 50 };
    let nb_zeros = safe.apply_turns(turns);
    println!("P1 = {}, P2 = {}", nb_zeros.0, nb_zeros.1);
}