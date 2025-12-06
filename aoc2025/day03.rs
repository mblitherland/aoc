use std::fs;

fn main() {
    let file = fs::read_to_string("day03.dat").unwrap();
    let mut sum = 0;
    for line in file.split("\n") {
        let length = line.len();
        if length > 0 {
            let digits: Vec<u64> = line.chars().map(|c| c.to_string().parse::<u64>().unwrap()).collect();
            let mut highest = 0;
            let mut position = 0;
            for i in 0..length - 1 {
                if digits[i] > highest {
                    highest = digits[i];
                    position = i;
                }
            }
            let mut next_highest = 0;
            for i in position + 1..length {
                if digits[i] > next_highest {
                    next_highest = digits[i];
                }
            }
            sum += highest * 10 + next_highest;
        }
    }
    println!("Sum: {}", sum);
}
