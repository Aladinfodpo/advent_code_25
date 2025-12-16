use std::env;
use std::fs;
use std::collections::HashSet;
use std::fmt;

#[derive(Eq, Hash, PartialEq)]
struct JBox{
    coords : Vec<u32>
}

impl JBox{
    fn from_string(string : String) -> Vec<JBox>{
        string.lines().map(|line| {JBox { coords: line.split(",").map(|number| number.parse().unwrap()).collect()}}).collect()
    }

    fn distance(&self, other : &JBox) -> f64 {
        (0..3).fold(0.0, |res, i| {res + (self.coords[i] as f64 - other.coords[i] as f64) * (self.coords[i] as f64 - other.coords[i] as f64) }).sqrt()
    }
}

impl fmt::Display for JBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.coords[0..self.coords.len()-1].iter().for_each(|x|{
            write!(f, "{},", x).unwrap();
        });
        write!(f, "{})", self.coords.last().unwrap())?;
        Ok(())
    }
}

fn get_sorted_pair(list : &Vec<JBox>) -> Vec<(& JBox, & JBox)>{
    let mut pairs = vec![];
    for i in 0..list.len(){
        for j in (i+1)..list.len(){
            pairs.push((&list[i], &list[j]));
        }
    }

    pairs.sort_by(|a, b|{return a.0.distance(a.1).total_cmp(&b.0.distance(b.1))});

    pairs
}

fn get_p1(pairs : &Vec<(& JBox, & JBox)>, n_pair : usize) -> u64{
    let mut circuits : Vec<HashSet<& JBox>> = Vec::new();
    for i in 0..n_pair{

        match (circuits.iter().position(|set| {set.contains(&pairs[i].0)}), circuits.iter().position(|set| {set.contains(&pairs[i].1)})){
            (None, None) => {circuits.push(HashSet::from([pairs[i].0, pairs[i].1]));},
            (Some(i0), Some(i1)) => { 
                if i0 != i1 {
                    circuits[i0] = circuits[i0].union(&circuits[i1]).map(|element| *element).collect::<HashSet<& JBox>>();
                    circuits.remove(i1);
                }
            },
            (Some(index), None) => {
                circuits[index].insert(pairs[i].1);
            },
            (None, Some(index)) => {
                circuits[index].insert(pairs[i].0);
            }
        };

        /*for (index, circuit) in circuits.iter().enumerate() {
            println!("Circuit {}/{} : ", index, circuits.len());
            for jbox in circuit {
                println!("{}", jbox);
            }
            println!("");
        }*/
    }

    circuits.sort_by(|a, b| {b.len().cmp(&a.len())});

    circuits[0..3].iter().fold(1, |res, x| {res * x.len() as u64})
}

fn get_p2(pairs : &Vec<(& JBox, & JBox)>, n_box : usize) -> u64{
    let mut circuits : Vec<HashSet<& JBox>> = Vec::new();
    let mut i = 0;
    loop {
        match (circuits.iter().position(|set| {set.contains(&pairs[i].0)}), circuits.iter().position(|set| {set.contains(&pairs[i].1)})){
            (None, None) => {circuits.push(HashSet::from([pairs[i].0, pairs[i].1]));},
            (Some(i0), Some(i1)) => { 
                if i0 != i1 {
                    circuits[i0] = circuits[i0].union(&circuits[i1]).map(|element| *element).collect::<HashSet<& JBox>>();
                    if circuits[i0].len() == n_box {
                        break pairs[i].0.coords[0] as u64 * pairs[i].1.coords[0] as u64
                    }
                    circuits.remove(i1);
                }
            },
            (Some(index), None) => {
                circuits[index].insert(pairs[i].1);
                if circuits[index].len() == n_box {
                    break pairs[i].0.coords[0] as u64 * pairs[i].1.coords[0] as u64
                }
            },
            (None, Some(index)) => {
                circuits[index].insert(pairs[i].0);
                if circuits[index].len() == n_box {
                    break pairs[i].0.coords[0] as u64 * pairs[i].1.coords[0] as u64
                }
            }  
        };

        i += 1;
    }
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    let list = JBox::from_string(get_file());
    println!("Parsing done.");
    let pairs =  get_sorted_pair(&list);
    println!("Sorting done.");
    println!("P1: {}", get_p1(&pairs, 1000));
    println!("P1: {}", get_p2(&pairs, list.len()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let string = concat!(
            "162,817,812\n",
            "57,618,57\n",
            "906,360,560\n",
            "592,479,940\n",
            "352,342,300\n",
            "466,668,158\n",
            "542,29,236\n",
            "431,825,988\n",
            "739,650,466\n",
            "52,470,668\n",
            "216,146,977\n",
            "819,987,18\n",
            "117,168,530\n",
            "805,96,715\n",
            "346,949,466\n",
            "970,615,88\n",
            "941,993,340\n",
            "862,61,35\n",
            "984,92,344\n",
            "425,690,689").to_string();
        let list = JBox::from_string(string);
        let pairs =  get_sorted_pair(&list);

        assert_eq!(pairs[0].0.coords[0], 162);
        assert_eq!(pairs[0].0.coords[1], 817);
        assert_eq!(pairs[0].0.coords[2], 812);

        assert_eq!(pairs[0].1.coords[0], 425);
        assert_eq!(pairs[0].1.coords[1], 690);
        assert_eq!(pairs[0].1.coords[2], 689);

        assert_eq!(pairs[1].0.coords[0], 162);
        assert_eq!(pairs[1].0.coords[1], 817);
        assert_eq!(pairs[1].0.coords[2], 812);

        assert_eq!(pairs[1].1.coords[0], 431);
        assert_eq!(pairs[1].1.coords[1], 825);
        assert_eq!(pairs[1].1.coords[2], 988);

        assert_eq!(get_p1(&pairs, 10), 40);
    }

    #[test]
    fn test_p2(){
        let string = concat!(
            "162,817,812\n",
            "57,618,57\n",
            "906,360,560\n",
            "592,479,940\n",
            "352,342,300\n",
            "466,668,158\n",
            "542,29,236\n",
            "431,825,988\n",
            "739,650,466\n",
            "52,470,668\n",
            "216,146,977\n",
            "819,987,18\n",
            "117,168,530\n",
            "805,96,715\n",
            "346,949,466\n",
            "970,615,88\n",
            "941,993,340\n",
            "862,61,35\n",
            "984,92,344\n",
            "425,690,689").to_string();
        let list = JBox::from_string(string);
        let pairs =  get_sorted_pair(&list);

        assert_eq!(get_p2(&pairs, list.len()), 25272);
    }
}