use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("day08.test").unwrap();
    let mut coords = vec![];
    for line in file.lines() {
        let values: Vec<i64> = line.split(",").map(|v| v.parse::<i64>().unwrap()).collect();
        coords.push(values);
    }
    let mut lengths = HashMap::new();
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            if i != j {
                lengths.insert((i, j), length(&coords[i], &coords[j]));
            }
        }
    }
    // lengths.sort_by()
    println!("lengths: {:?}", lengths);
}

fn length(v1: &Vec<i64>, v2: &Vec<i64>) -> f64 {
    f64::sqrt(((v1[0] - v2[0]) * (v1[0] - v2[0]) +
        (v1[1] - v2[1]) * (v1[1] - v2[1]) +
        (v1[2] - v2[2]) * (v1[2] - v2[2])) as f64)
}
