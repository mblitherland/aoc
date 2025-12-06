use std::fs;

fn main() {
    let file = fs::read_to_string("day05.dat").unwrap();
    let mut ranges = vec![];
    let mut inventory = vec![];

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }
        if line.contains("-") {
            let (before, after) = line.split_once("-").unwrap();
            ranges.push((before.parse::<u64>().unwrap(), after.parse::<u64>().unwrap()));
        } else {
            inventory.push(line.parse::<u64>().unwrap());
        }
    }
    println!("ranges: {:?}", ranges);
    println!("inventory: {:?}", inventory);
    let mut count = 0;
    'OUTER: for i in inventory {
        for r in &ranges {
            if i >= r.0 && i <= r.1 {
                count += 1;
                continue 'OUTER
            }
        }
    }
    println!("Total count: {}", count);
}
