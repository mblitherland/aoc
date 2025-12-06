use std::fs;

fn main() {
    let mut num = 50;
    let mut count = 0;
    let data = fs::read_to_string("day01.dat").unwrap();
    for line in data.split("\n") {
        if line.len() > 1 {
            let (_, d) = line.split_at(1);
            let v = d.parse::<i32>().unwrap();
            if line.starts_with("L") {
                num -= v;
                while num < 0 {
                    num += 100;
                }
            } else if line.starts_with("R") {
                num += v;
                while num > 99 {
                    num -= 100;
                }
            }
        }
        if num == 0 {
            count += 1;
        }
    }
    println!("Answer is: {}", count);
}

