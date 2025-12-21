use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::u32;

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

struct MachineParsed {
    lights : Vec<bool>,
    buttons : Vec<Vec<bool>>,
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
                buttons : parts[1..(parts.len()-1)].iter().map(
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

fn uncompress(bits : u32, n : usize) -> Vec<bool>{
    (0..n).rev().map(|digit| {bits & (1 << digit) > 0 }).collect()
}

impl Machine {    
    fn find_min_nb_press(&self) -> u32{
        let mut min_press = u32::MAX;

        for i in 0..(1<<self.buttons.len()){
            if self.press(i) == self.lights{
                min_press = min_press.min(i.count_ones());
            }
        }
        min_press
    }
}

fn solve_p1(machines : &Vec<Machine>) -> u32{
    machines.iter().fold(0, |res, machine| res + machine.find_min_nb_press())
}

struct Machine {
    buttons : Vec<Vec<u32>>,
    buttons_encoded : Vec<u32>,
    joltages : Vec<u32>,
    lights : u32,
}

impl Machine {
    fn from_mp(parsed : &MachineParsed) -> Machine{
        Machine { 
            lights: compress(&parsed.lights),
            joltages: parsed.joltage.clone(), 
            buttons: parsed.buttons.iter().map(|vec: &Vec<bool>| vec.iter().map(|bit| if *bit {1} else {0}).collect()).collect(),
            buttons_encoded: parsed.buttons.iter().map(|vec: &Vec<bool>| compress(vec)).collect()
        }
    }

    fn reduce(&self, new_joltages : &Vec<u32>) -> Machine{
        Machine { 
            lights: self.lights.clone(),
            joltages: new_joltages.clone(), 
            buttons: self.buttons.clone(),
            buttons_encoded: self.buttons_encoded.clone(),
        }
    }

    fn from_mps(parsed : &Vec<MachineParsed>) -> Vec<Machine>{
        parsed.iter().map(|parsed| {Machine::from_mp(parsed)}).collect()
    }

    //return new state after pushing buttons
    fn press(&self, index : u32) -> u32{
        let mut res : u32 = 0;
        for i in 0..self.buttons_encoded.len(){
            if (index & (1 << i)) != 0 {
                res ^= self.buttons_encoded[i];
            }
        }
        res
    }

    fn press_joltages(&self, index : u32) -> Option<Vec<u32>>{
        let mut res  = self.joltages.clone();
        for i in 0..self.buttons_encoded.len(){
            if (index & (1 << i)) != 0 {
                let decompressed = uncompress(self.buttons_encoded[i], res.len());
                for j in 0..res.len(){
                    if(res[j] == 0 && decompressed[j]){return None;}
                    res[j] -= if decompressed[j] {1} else {0};
                }
            }
        }
        Some(res)
    }

    fn i_have_cheated_p2(&self) -> u32{
        let n = self.buttons.len();

        // create inputs possible
        let mut hash_possible : HashMap<u32, Vec<u32>> = HashMap::new();
        for i in 0..(1<<n){
            hash_possible.entry(self.press(i)).or_insert(vec![]).push(i);
        }

        self.solve_reduce(&hash_possible).unwrap()
    }

    fn solve_reduce(&self, hash_possible : &HashMap<u32, Vec<u32>>) -> Option<u32>{
        if self.joltages.iter().all(|x| *x == 0){Some(0)}
        else{
            let compressed_to_even = compress(&self.joltages.iter().map(|nb|{nb % 2 == 1}).collect());
            let eveners = hash_possible.get(&compressed_to_even)?;

            let mut min_nb = None;
            for evener in eveners{
                let pressed = self.press_joltages(*evener);
                if (!matches!(pressed, None)){
                    match self.reduce(&pressed.unwrap().iter().map(|nb| { nb >> 1}).collect()).solve_reduce(hash_possible){
                        None => (),
                        Some(value) => {min_nb = Some(min_nb.unwrap_or(u32::MAX).min(2*value+evener.count_ones()));}
                    }
                }
            }
            min_nb
        }
    }

}

fn main() {
    let parsed = MachineParsed::from_string(get_file());
    let machines = Machine::from_mps(&parsed);
    println!("P1: {}", solve_p1(&machines));

    let machines_p2 = Machine::from_mps(&parsed);
    println!("P2: {}", machines_p2.iter().enumerate().fold(0, |res, (index, machine)|{ res + machine.i_have_cheated_p2()}));
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

    #[test]
    fn test_exemple_p2() {
        let string = concat!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}\n",
        ).to_string();
        let parsed = MachineParsed::from_string(string);
        let machines = Machine::from_mps(&parsed);
        assert_eq!(machines.iter().fold(0, |res, machine|{res + machine.solve_from_0(norme_1, u32::MAX)}), 33);
        assert_eq!(machines.iter().fold(0, |res, machine|{res + machine.solve_from_0(norme_2, u32::MAX)}), 33);
        assert_eq!(machines.iter().fold(0, |res, machine|{res + machine.i_have_cheated_p2()}), 33);
    }
}

// End here








// Non working attempt using weighted A* 
// Only work for small inputs (40ish^6 max) 

impl Machine {
    fn get_max_dp(&self)-> f64{
        self.buttons.iter().max_by(|a, b| a.iter().sum::<u32>().cmp(&b.iter().sum::<u32>())).unwrap().iter().sum::<u32>() as f64
    }

    fn get_mean_dp(&self)-> f64{
        self.buttons.iter().fold(0.0f64, |res, butto| res + butto.iter().sum::<u32>() as f64) / self.buttons.len() as f64
    }

    fn solve_from_0(&self, h : fn(&Vec<u32>, &Vec<u32>) -> f64, n_max : u32) ->u32{
        self.solve_from(0, h, n_max)
    }

    fn solve_from(&self, start : u32, h : fn(&Vec<u32>, &Vec<u32>) -> f64, n_max : u32) ->u32{
        self.solve_a_star(&vec![start; self.joltages.len()], h, n_max)
    }

    fn solve_from_percent(&self, percent : u32, h : fn(&Vec<u32>, &Vec<u32>) -> f64, n_max : u32) ->u32{
        self.solve_a_star(&self.joltages.iter().map(|nb|{nb*percent/100}).collect(), h, n_max)
    }

    fn get_simpler(&self, percent : u32) -> Machine{
        Machine { lights: self.lights, buttons_encoded : self.buttons_encoded.clone(), buttons: self.buttons.clone(), joltages: self.joltages.iter().map(|nb|{nb*percent/100}).collect() }
    }

    fn solve_a_star(&self, start : &Vec<u32>, h : fn(&Vec<u32>, &Vec<u32>) -> f64, n_max : u32) -> u32{
        let mut open = HashSet::from([start.clone()]);

        let mut gscore = HashMap::from([(start.clone(), 0)]);
        let mut fscore = HashMap::from([(start.clone(), h(&start, &self.joltages))]);

        let mut nb_iter = 0;

        while !open.is_empty(){
            nb_iter += 1;
            let current = open.iter().min_by(|a, b| fscore.get(*a).unwrap_or(&f64::INFINITY).partial_cmp(&fscore.get(*b).unwrap_or(&f64::INFINITY)).unwrap()).unwrap().clone();
            open.remove(&current);

            if (0..current.len()).any(|index|{current[index] > self.joltages[index]}){
                continue;
            }

            if *current == self.joltages {return gscore[&current];}
            if (nb_iter > n_max){
                return 0;
            }
            //println!("{}", norme_1(&current, &self.joltages));
            //for (index, cur) in current.iter().enumerate(){ print!("{},", self.joltages[index] - cur);}println!("");

            let new_score = gscore[&current] + 1;

            let max_index = diff_argmax(&current, &self.joltages);
            for button in self.buttons.iter(){
                if button[max_index] > 0 {
                    let new_state: Vec<u32> = current.iter().enumerate().map(|(index, nb)|{nb + button[index]}).collect();
                    
                    if new_score < *gscore.get(&new_state).unwrap_or(&u32::MAX){
                        *gscore.entry(new_state.clone()).or_insert(0) = new_score;
                        *fscore.entry(new_state.clone()).or_insert(0.0) = new_score as f64 + h(&new_state, &self.joltages);
                        open.insert(new_state);
                    }
                }
            }
        }
        0
    }

    fn solve_new(&self) -> u32{
        let first_res = self.solve_from_0(norme_direction, 300000);
        if first_res > 0 {
            first_res
        }else{
            let res_20 = self.get_simpler(25).solve_from_0(norme_direction, u32::MAX);
            if(res_20 == 0){println!("No path found"); 0}
            else{
                res_20*3 + self.solve_from_percent(75,norme_direction, u32::MAX)
            }
        }
    }

}


fn n2(a: &Vec<u32>) -> f64{
    a.iter().fold(0.0, |res, val|{res + (val * val) as f64}).sqrt()
}

fn diff_argmax(a: &Vec<u32>, b: &Vec<u32>) -> usize{
    (0..a.len()).fold((0, 0u32), |res, index|{
        let new_res = a[index].abs_diff(b[index]);
        if new_res > res.1 {(index, new_res)} else {res}
    }).0
}

fn norme_1(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
    (0..a.len()).fold(0, |res, index|{res + a[index].abs_diff(b[index])}) as f64
}

fn norme_2(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
    (0..a.len()).fold(0.0, |res, index|{res + (a[index] as f64 -b[index] as f64)*(a[index] as f64 -b[index] as f64)}).sqrt()
}

fn dot(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
    (0..a.len()).fold(0, |res, index|{res + a[index] * b[index]}) as f64
}

fn norme_direction(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
    norme_2(a, b) * 5.0 / (dot(a, b) as f64 / n2(a) / n2(b))
}

fn norme_max_<const N: usize>(a: &Vec<u32>, b: &Vec<u32>) -> f64 {
    let mut diff : Vec<u32> = (0..a.len()).map(|index|{a[index].abs_diff(b[index])}).collect();
    diff.sort();
    n2(&diff.into_iter().rev().take(N).collect())
}