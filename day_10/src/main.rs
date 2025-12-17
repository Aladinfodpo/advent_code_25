use std::env;
use std::fs;

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

struct MachineParsed {
    lights : Vec<bool>,
    button : Vec<Vec<bool>>,
    joltage: Vec<u32>,
}

impl MachineParsed {
    fn from_string(string : String) ->Vec<MachineParsed>{
        string.replace("[","").replace("]","")
              .replace("(","").replace(")","")
              .replace("{","").replace("}","")
              .lines().map(|line|{
            let parts = line.split(' ').collect::<Vec<&str>>();
            let nb_lights = parts[0].len();
            MachineParsed{ 
                lights : parts[0].chars().map(|light|{light == '#'}).collect(),
                button : parts[1..(parts.len()-1)].iter().map(
                    |button_str|{
                        let mut button = vec![false; nb_lights];
                        button_str.split(",").for_each(
                            |input|{
                                button[input.parse::<usize>().unwrap()] = true;
                            }
                        );
                        button
                    }).collect(),
                joltage : parts.last().unwrap().split(',').map(|jolt_str|{jolt_str.parse::<u32>().unwrap()}).collect(),
            }
        }).collect()
    }
}

fn compress(vec : &Vec<bool>) -> u32{
    vec.iter().fold(0, |res, digit| {res * 2 + if *digit {1} else {0}})
}

struct Machine {
    lights : u32,
    button : Vec<u32>,
    joltage: Vec<u32>,
}

impl Machine {
    fn from_mp(parsed : &MachineParsed) -> Machine{
        Machine { lights: compress(&parsed.lights), button: parsed.button.iter().map(|vec: &Vec<bool>| compress(vec)).collect(), joltage: parsed.joltage.clone() }
    }

    fn from_mps(parsed : &Vec<MachineParsed>) -> Vec<Machine>{
        parsed.iter().map(|parsed| {Machine::from_mp(parsed)}).collect()
    }
    
    fn find_min_nb_press(&self) -> u32{
        let mut min_press = u32::MAX;

        for i in 0..(1<<self.button.len()){
            if self.press(i) == self.lights{
                min_press = min_press.min(i.count_ones());
            }
        }
        min_press
    }

    fn press(&self, index : u32) -> u32{
        let mut res : u32 = 0;
        for i in 0..self.button.len(){
            if (index & (1 << i)) != 0 {
                res ^= self.button[i];
            }
        }
        res
    }
}

fn solve_p1(machines : &Vec<Machine>) -> u32{
    machines.iter().fold(0, |res, machine| res + machine.find_min_nb_press())
}

fn main() {
    let parsed = MachineParsed::from_string(get_file());
    let machines = Machine::from_mps(&parsed);
    
    println!("P1: {}", solve_p1(&machines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple() {
        let string = concat!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n",
        ).to_string();
        let parsed = MachineParsed::from_string(string);
        let machines = Machine::from_mps(&parsed);
        assert_eq!(solve_p1(&machines), 7);
    }
}