use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Clone, Copy, Debug)]
struct Set {
    r: u64,
    g: u64,
    b: u64,
}

fn main() {
    let input_path = env::var("aoc_2023_02_path").unwrap() + "/input.txt";
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    let mut sum = 0u64;
    let mut min_sum = 0u64;

    for (i, line) in reader.lines().enumerate() {
        let line: String = line.unwrap().split(": ").skip(1).collect();
        let data1: Vec<&str> = line.split(";").map(|x| x.trim()).collect();
        let mut game_sets: Vec<Set> = Vec::with_capacity(data1.len());
        for set in data1 {
            let colors_data: Vec<&str> = set.split(",").map(|x| x.trim()).collect();
            let color_set = parse_set(&colors_data);
            game_sets.push(color_set);
        }
        let mut max_r = 0u64;
        let mut max_g = 0u64;
        let mut max_b = 0u64;
        for set in &game_sets {
            if set.r > max_r {
                max_r = set.r;
            } else {
            }
            if set.g > max_g {
                max_g = set.g;
            }
            if set.b > max_b {
                max_b = set.b;
            }
        }
        min_sum = min_sum + max_r * max_g * max_b;
        if max_r <= 12 && max_g <= 13 && max_b <= 14 {
            sum = sum + (i as u64) + 1;
        }
        println!("r{}g{}b{}", max_r, max_g, max_b);
    }
    println!("part 1: {}, part 2: {}", sum, min_sum);
}
fn parse_set(sets: &[&str]) -> Set {
    let mut res = Set { r: 0, g: 0, b: 0 };
    for set in sets {
        let a: Vec<&str> = set.split(" ").collect();
        // println!("{:?}", a);
        let number = u64::from_str_radix(a[0], 10).unwrap();
        let color = a[1];
        if color == "red" {
            res.r = number;
        } else if color == "green" {
            res.g = number;
        } else if color == "blue" {
            res.b = number;
        }
    }

    return res;
}
