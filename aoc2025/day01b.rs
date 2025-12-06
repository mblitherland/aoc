use std::fs;

fn main() {
    let mut num = 50;
    let mut count = 0;
    let data = fs::read_to_string("day01.test").unwrap();
    for line in data.split("\n") {
        if line.len() > 1 {
            println!("n: {} line: {}", num, line);
            let (_, d) = line.split_at(1);
            let v = d.parse::<i32>().unwrap();
            if line.starts_with("L") {
                let start = num;
                num -= v;
                while num < 0 {
                    num += 100;
                    println!("* n1: {} => {}", start, num);
                    if start != 0 {
                        count += 1;
                    }
                }
            } else if line.starts_with("R") {
                let start = num;
                num += v;
                while num > 99 {
                    num -= 100;
                    println!("* n2: {} => {}", start, num);
                    if start != 0 {
                        count+= 1;
                    }
                }
            }
            if num == 0 {
                println!("* n3: {}", num);
                count += 1;
            }
        }
    }
    println!("Answer is: {}", count);
}

