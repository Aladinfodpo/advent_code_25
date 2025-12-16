use std::env;
use std::fs;
use std::collections::HashSet;

struct Manifold {
    x_start: usize,
    x : usize,
    y : usize,
    splitters : Vec<Vec<usize>>,
}
  
impl Manifold {
    fn from_string(string : String) -> Manifold {
        let lines : Vec<&str> = string.lines().collect();
        let start = lines[0].find('S').unwrap();

        let splitters : Vec<Vec<usize>> = lines.iter().map(
            |line| {
                line.chars().enumerate().fold(vec![], |mut v : Vec<usize>, ic| {if ic.1 == '^' {v.push(ic.0)} v})
            }
        ).collect();

        Manifold {
            x_start : start,
            x: lines[0].len(),
            y: lines.len(),
            splitters: splitters,
        }
    }

    fn count_split(& self) -> u32{
        let mut res : u32 = 0;
        let mut rays  = HashSet::from([self.x_start]);

        for y in 0..self.y {
            rays = rays.iter().fold(HashSet::new(), 
                |mut v : HashSet<usize>, ray_x| {
                    if self.splitters[y].contains(ray_x) {
                        v.insert(*ray_x-1);
                        v.insert(*ray_x+1);
                        res += 1;
                    } else {
                        v.insert(*ray_x);
                    } 
                    v
                }
            )
        }

        res
    }

    #[allow(dead_code)]
    // Naive first attempt, right but too inefficient to get the result
    fn count_timelines_naive(& self) -> u64 {
        let mut res: u64 = 0;

        let mut x_ray = self.x_start;
        let mut y_ray = 0;

        let mut splitters: Vec<(usize, usize)> = vec![];
    
        loop {
            if self.splitters[y_ray].contains(&x_ray){
                splitters.push((x_ray, y_ray));
                x_ray -= 1;
            }

            y_ray += 1;

            if y_ray == self.y {
                res += 1;

                if splitters.is_empty() {break res}

                let last_checkpoint = splitters.pop().unwrap();
                x_ray = last_checkpoint.0 + 1;
                y_ray = last_checkpoint.1;
            }
        }
    }
    
    fn count_timelines(& self) -> u64 {
        let mut rays  =  Vec::from_iter(std::iter::repeat(0).take(self.x));
        rays[self.x_start] = 1;

        for y in 1..self.y {
            let mut new_rays:Vec<u64> = Vec::from_iter(std::iter::repeat(0).take(self.x));
            
            rays.iter().enumerate().for_each(|(i, weight)|{
                if self.splitters[y].contains(&i){
                    new_rays[i-1] += weight;
                    new_rays[i+1] += weight;
                }else{
                    new_rays[i] += weight;
                }
            });

            rays = new_rays;
        }

        rays.iter().fold(0u64, |res, ray| res + ray)
    }
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    let manifold = Manifold::from_string(get_file());
    println!("P1: {}", manifold.count_split());
    println!("P2: {}", manifold.count_timelines());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple_p1() {
        let string = concat!(
            ".......S.......\n",
            "...............\n",
            ".......^.......\n",
            "...............\n",
            "......^.^......\n",
            "...............\n",
            ".....^.^.^.....\n",
            "...............\n",
            "....^.^...^....\n",
            "...............\n",
            "...^.^...^.^...\n",
            "...............\n",
            "..^...^.....^..\n",
            "...............\n",
            ".^.^.^.^.^...^.\n",
            "...............\n"
        ).to_string();
        let manifold = Manifold::from_string(string);
        assert_eq!(manifold.count_split(), 21);
    }

    #[test]
    fn test_exemple_p2_naive() {
        let string = concat!(
            ".......S.......\n",
            "...............\n",
            ".......^.......\n",
            "...............\n",
            "......^.^......\n",
            "...............\n",
            ".....^.^.^.....\n",
            "...............\n",
            "....^.^...^....\n",
            "...............\n",
            "...^.^...^.^...\n",
            "...............\n",
            "..^...^.....^..\n",
            "...............\n",
            ".^.^.^.^.^...^.\n",
            "...............\n"
        ).to_string();
        let manifold = Manifold::from_string(string);
        assert_eq!(manifold.count_timelines_naive(), 40);
    }

    #[test]
    fn test_exemple_p2() {
        let string = concat!(
            ".......S.......\n",
            "...............\n",
            ".......^.......\n",
            "...............\n",
            "......^.^......\n",
            "...............\n",
            ".....^.^.^.....\n",
            "...............\n",
            "....^.^...^....\n",
            "...............\n",
            "...^.^...^.^...\n",
            "...............\n",
            "..^...^.....^..\n",
            "...............\n",
            ".^.^.^.^.^...^.\n",
            "...............\n"
        ).to_string();
        let manifold = Manifold::from_string(string);
        assert_eq!(manifold.count_timelines(), 40);
    }
}
