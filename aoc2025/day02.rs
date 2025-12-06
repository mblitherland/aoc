use std::fs;

fn main() {
    let mut sum = 0;
    let data = fs::read_to_string("day02.dat").unwrap();
    for line in data.split(",") {
        if line.len() > 0 {
            if let Some((a, b)) = line.trim().split_once("-") {
                let start = a.parse::<i128>().unwrap();
                let end = b.parse::<i128>().unwrap();
                for i in start..end + 1 {
                    let current = i.to_string();
                    let len = current.len();
                    if len % 2 == 0 {
                        let (front, back) = current.split_at(len / 2);
                        if front == back {
                            println!("double {}", current);
                            sum += i;
                        }
                    }
                }
            }
        }
    }
    println!("The answer is: {}", sum);
}
