use std::env;
use std::fs;

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

struct Corner {
    x : u64,
    y : u64,
}

impl Corner {
    fn get_area(&self, other : &Corner) -> u64{
        (self.x.max(other.x) - self.x.min(other.x) + 1) * (self.y.max(other.y) - self.y.min(other.y) + 1)
    }

    fn from_string(string : &String) -> Vec<Corner>{
        string.lines().map(|line| {
            let corners : Vec<&str>= line.split(',').collect();
            Corner{ x : corners[0].parse().unwrap(), y : corners[1].parse().unwrap()}
        }).collect()
    }
}


fn does_intersect_point(s1 : (&Corner, &Corner), corner: &Corner) -> bool{

    if s1.0.x as i64 - s1.1.x as i64 != 0 {
        // s1 horizontal
        false
    }else{
        // s1 vertical
        s1.0.x as f64 <= corner.x as f64 + 0.5 && s1.0.y.min(s1.1.y) as f64 <= corner.y as f64 + 0.5 && s1.0.y.max(s1.1.y) as f64 >= corner.y as f64 + 0.5
    }
}

fn check_corner_inside(corner : Corner, corners : &Vec<Corner>) -> bool{
    let mut nb_hit = 0;
    for i in 0..corners.len(){
        if corner.x >= corners[i].x.min(corners[(i+1)%corners.len()].x) && corner.x <= corners[i].x.max(corners[(i+1)%corners.len()].x) 
        && corner.y >= corners[i].y.min(corners[(i+1)%corners.len()].y) && corner.y <= corners[i].y.max(corners[(i+1)%corners.len()].y){
            return true;
        }
    }
    for i in 0..corners.len(){
        nb_hit += if does_intersect_point((&corners[i], &corners[(i+1)%corners.len()]), &corner){1} else {0};
    }
    nb_hit % 2 == 1
}

fn get_sorted_rectangles(corners : &Vec<Corner>) -> Vec<(u64, &Corner, &Corner)>{
    let mut res = vec![];
    for i in 0..corners.len(){ 
        for j in (i+1)..corners.len(){
            res.push((corners[i].get_area(&corners[j]), &corners[i], &corners[j]));
        }
    }
    res.sort_by(|(area_a, _, _), (area_b, _, _)|{area_b.cmp(area_a)});
    res
}

fn get_max_area_rectangle_p1(sorted: &Vec<(u64, &Corner, &Corner)>) -> u64{
    sorted[0].0
}

fn get_max_area_rectangle_p2(sorted: &Vec<(u64, &Corner, &Corner)>, corners : &Vec<Corner>, start : usize) -> u64{
    let mut max_area = 0;
    for i in start..sorted.len(){ 
        println!("{}/{}", i, sorted.len());
        let mut is_ok = true;

        let corner1 = sorted[i].1;
        let corner2 = sorted[i].2;

        for x in corner1.x.min(corner2.x)..=corner1.x.max(corner2.x) {
            is_ok = is_ok && check_corner_inside(Corner { x, y : corner1.y }, &corners) && check_corner_inside(Corner { x, y : corner2.y }, &corners);
            if !is_ok {break;}
        }

        for y in corner1.y.min(corner2.y)..=corner1.y.max(corner2.y){
            is_ok = is_ok && check_corner_inside(Corner { x : corner1.x, y }, &corners) && check_corner_inside(Corner { x : corner2.x, y }, &corners);
            if !is_ok {break;}
        }

        if is_ok {
            max_area = sorted[i].0;
            break;
        }
    }
    max_area
}

fn main() {
    let start_at= env::args().collect::<Vec<String>>().get(1).unwrap_or(&"0".to_string()).parse::<usize>().unwrap_or(0);
    let corners = Corner::from_string(&get_file());
    let sorted = get_sorted_rectangles(&corners);
    println!("P1: {}", get_max_area_rectangle_p1(&sorted));
    println!("P2: {}", get_max_area_rectangle_p2(&sorted, &corners, start_at));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let string = concat!(
            "7,1\n",
            "11,1\n",
            "11,7\n",
            "9,7\n",
            "9,5\n",
            "2,5\n",
            "2,3\n",
            "7,3\n",
        ).to_string();
        assert_eq!(get_max_area_rectangle_p1(&get_sorted_rectangles(&Corner::from_string(&string))), 50);
    }

    #[test]
    fn test_inside() {
        let string = concat!(
            "7,1\n",
            "11,1\n",
            "11,7\n",
            "9,7\n",
            "9,5\n",
            "2,5\n",
            "2,3\n",
            "7,3\n",
        ).to_string();
        let corners = Corner::from_string(&string);

        let test = concat!(
            "..............\n",
            ".......#XXX#..\n",
            ".......XXXXX..\n",
            "..#XXXX#XXXX..\n",
            "..XXXXXXXXXX..\n",
            "..#XXXXXX#XX..\n",
            ".........XXX..\n",
            ".........#X#..\n",
            "..............\n").to_string();
        let test_lines = test.lines().collect::<Vec<&str>>();
        
        assert_eq!(check_corner_inside(Corner { x: 0, y: 0 }, &corners), false);
        assert_eq!(check_corner_inside(Corner { x: 3, y: 4 }, &corners), true);
        assert_eq!(check_corner_inside(Corner { x: 3, y: 3 }, &corners), true);
        assert_eq!(check_corner_inside(Corner { x: 2, y: 3 }, &corners), true);



        for y in 0..test_lines.len(){
           for x in 0..test_lines[0].len(){
                println!("x = {}, y = {}, char = {}", x, y, test_lines[y].chars().nth(x).unwrap());
                assert_eq!(check_corner_inside(Corner { x: x as u64, y: y as u64}, &corners), test_lines[y].chars().nth(x) != Some('.'));
            } 
        }
    }

    #[test]
    fn test_p2() {
        let string = concat!(
            "7,1\n",
            "11,1\n",
            "11,7\n",
            "9,7\n",
            "9,5\n",
            "2,5\n",
            "2,3\n",
            "7,3\n",
        ).to_string();
        let corners = Corner::from_string(&string);
        let sorted = get_sorted_rectangles(&corners);

        assert_eq!(get_max_area_rectangle_p2(&sorted, &corners, 0), 24);
    }
}