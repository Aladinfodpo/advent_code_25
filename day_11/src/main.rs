use std::env;
use std::fs;
use std::collections::HashMap;

struct Device {
   outs : Vec<String>,
}

struct WeigtedDevice {
   outs : Vec<String>,
   done : bool,
   weight : u64,
}

impl Device {
    fn from_string(string : String) -> HashMap<String, Device>{
        HashMap::from_iter(string.replace(':', "").lines().map(|line| {
            let parts :Vec<&str>= line.split(' ').collect();
            (parts[0].to_string(), Device{
                outs : parts[1..].iter().map(|str| {str.to_string()}).collect(),
            })
        }))
    }

    fn to_weighted(&self) -> WeigtedDevice{
        WeigtedDevice { weight : 0u64, done : false, outs: self.outs.iter().map(|s|{s.clone()}).collect() }
    }
}

fn print_devices(list : &HashMap<String, Device>){
    for (from, to) in list.iter(){
        print!("{} :", from);
        for out in to.outs.iter(){
            print!(" {}", out);
        }
        println!("");
    }
}

fn print_wdevices(list : &HashMap<String, WeigtedDevice>){
    for (from, to) in list.iter(){
        print!("{} = {}:", from, to.weight);
        for out in to.outs.iter(){
            print!(" {}", out);
        }
        println!("");
    }
}

fn to_weighted(list : &HashMap<String, Device>) -> HashMap<String, WeigtedDevice>{
    HashMap::from_iter(list.iter().map(|(k, v)| {(k.clone(), v.to_weighted())}))
}

fn reverse_device(list : &HashMap<String, Device>) -> HashMap<String, Device>{
    let mut res :HashMap<String, Device> = HashMap::new();

    for (from, to) in list{
        for out in to.outs.iter(){
            res.entry(out.clone()).or_insert(Device{ outs : vec![]}).outs.push(from.clone());
        }
    }

    res
}

fn fill_device(list : &HashMap<String, Device>, rev : &HashMap<String, Device>, end : &str) -> HashMap<String, WeigtedDevice>{
    let mut waiting = vec![String::from("out")];
    let mut res : HashMap<String, WeigtedDevice> = to_weighted(list);
    
    res.insert(String::from("out"),WeigtedDevice{ weight: 0u64, done : false, outs : vec![String::new()]});

    res.get_mut(end).unwrap().weight = 1u64;

    while !waiting.is_empty() {
        let current = &waiting.pop().unwrap();

        if res[current].outs.iter().any(|x|{res.contains_key(x) && !res[x].done}){
            res[current].outs.iter().for_each(|x|{if !res[x].done{waiting.push(x.clone());}});
            continue;
        }
        if res[current].done { continue; }

        res.get_mut(current).unwrap().done = true;

        if rev.contains_key(current) {
            let precs = rev[current].outs.clone();
            for prec in precs{
                res.get_mut(&prec).unwrap().weight += res[current].weight;
                waiting.push(prec);
            }
        }
    }
    res
}

fn get_p1(devices : &HashMap<String, Device>) -> u64{
    let reversed = reverse_device(&devices);
    let weighted = fill_device(&devices, &reversed, "out");

    weighted[&String::from("you")].weight
}

fn get_p2(devices : &HashMap<String, Device>) -> u64{
    let reversed = reverse_device(&devices);
    let weighted_out = fill_device(&devices, &reversed, "out");
    let weighted_dac = fill_device(&devices, &reversed, "dac");
    let weighted_fft = fill_device(&devices, &reversed, "fft");

    let svr_dac = weighted_dac[&String::from("svr")].weight;
    let dac_fft = weighted_fft[&String::from("dac")].weight;
    let fft_out = weighted_out[&String::from("fft")].weight;

    let svr_fft = weighted_fft[&String::from("svr")].weight;
    let fft_dac = weighted_dac[&String::from("fft")].weight;
    let dac_out = weighted_out[&String::from("dac")].weight;

    svr_dac*dac_fft*fft_out + svr_fft*fft_dac*dac_out
}

fn get_file() -> String {
    let exe_path = env::current_exe().expect("Failed to get exe path");
    let filename = exe_path.parent().unwrap().join("input.txt");
    fs::read_to_string(filename).expect("File is missing")
}

fn main() {
    let devices = Device::from_string(get_file());
    println!("P1: {}", get_p1(&devices));
    println!("P2: {}", get_p2(&devices));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let string = concat!(
            "aaa: you hhh\n",
            "you: bbb ccc\n",
            "bbb: ddd eee\n",
            "ccc: ddd eee fff\n",
            "ddd: ggg\n",
            "eee: out\n",
            "fff: out\n",
            "ggg: out\n",
            "hhh: ccc fff iii\n",
            "iii: out\n",
        ).to_string();

        let devices = Device::from_string(string);
        assert_eq!(get_p1(&devices), 5);
    }
    
    #[test]
    fn test_p2() {
        let string = concat!(
            "svr: aaa bbb\n",
            "aaa: fft\n",
            "fft: ccc\n",
            "bbb: tty\n",
            "tty: ccc\n",
            "ccc: ddd eee\n",
            "ddd: hub\n",
            "hub: fff\n",
            "eee: dac\n",
            "dac: fff\n",
            "fff: ggg hhh\n",
            "ggg: out\n",
            "hhh: out\n",
        ).to_string();

        let devices = Device::from_string(string);
        assert_eq!(get_p2(&devices), 2);
    }
}