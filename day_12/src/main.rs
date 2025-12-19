use std::env;
use std::fs;

struct Shape {
    shape : u32,
}

struct Grid {
    x : u32,
    y : u32,
    shapes_nb : Vec<u32>
}

impl Grid {
    fn from_string(string : &str) -> Grid {
        let mut it = string.split(':');
        let mut it_size = it.next().unwrap().split('x').map(|s| {s.parse().unwrap()});

        Grid{x: it_size.next().unwrap(), y: it_size.next().unwrap(), shapes_nb: it.next().unwrap().split(' ').filter(|line| { !line.is_empty()}).map(|s|{s.parse().unwrap()}).collect()}
    }
    
    fn does_fit_naive(&self, shapes : &Vec<Shape>) -> bool{
        shapes.iter().enumerate().fold(0, |res, (index, shape)|{res + shape.get_area()*self.shapes_nb[index]}) <= self.get_area()
    }

    fn get_area(& self) -> u32{
        self.x * self.y
    }
}

impl Shape {
    fn from_string(strings : &[&str]) -> Shape{
        Shape{shape : (String::new()+strings[0]+strings[1]+strings[2]).chars().fold(0, |res, c|{res*2 + if c == '#' {1} else {0}})}
    }

    fn get_area(& self) -> u32{
        self.shape.count_ones()
    }
}

fn get_p1_naive(shapes : &Vec<Shape>, grids : &Vec<Grid>) -> u32{
    grids.iter().fold(0, |res, grid| {res + if grid.does_fit_naive(shapes) {1} else {0} })
}

fn vec_from_string(string: String) -> (Vec<Shape>, Vec<Grid>){    
    let shape_part = string.lines().take(30).filter(|line| { !line.contains(":") && !line.is_empty()}).collect::<Vec<&str>>().chunks(3).map(|rows| {Shape::from_string(rows)}).collect();
    let grids_part = string.lines().skip(30).map(|s|{Grid::from_string(s)}).collect();
    (shape_part, grids_part)
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    let (shapes, grids) = vec_from_string(get_file());
    println!("P1: {}/{}", get_p1_naive(&shapes, &grids), grids.len());
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_range() {
        let string = concat!(
            "0:\n",
            "###\n",
            "##.\n",
            "##.\n",
            "\n",
            "1:\n",
            "###\n",
            "##.\n",
            ".##\n",
            "\n",
            "2:\n",
            ".##\n",
            "###\n",
            "##.\n",
            "\n",
            "3:\n",
            "##.\n",
            "###\n",
            "##.\n",
            "\n",
            "4:\n",
            "###\n",
            "#..\n",
            "###\n",
            "\n",
            "5:\n",
            "###\n",
            ".#.\n",
            "###\n",
            "\n",
            "4x4: 0 0 0 0 2 0\n",
            "12x5: 1 0 1 0 2 2\n",
            "12x5: 1 0 1 0 3 2\n",
        ).to_string();

        let (shapes, grids) = vec_from_string(string);
        assert_eq!(get_p1_naive(&shapes, &grids), 2);
    }
}