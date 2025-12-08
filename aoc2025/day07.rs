use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("day07.dat").unwrap();

    let mut beam_columns: HashMap<usize, bool> = HashMap::new();
    let mut count = 0;
    
    let mut lines = file.lines();
    let first_line = lines.next().unwrap();
    for (i, c) in first_line.char_indices() {
        if c == 'S' {
            beam_columns.insert(i, true);
        }
    }
    println!("{}", first_line);

    while let Some(line) = lines.next() {
        let chars: Vec<char> = line.chars().collect();

        for i in 0..chars.len() {
            let mut beam = false;
            if let Some(_) = beam_columns.get(&i) {
                beam = true;
            }

            if *chars.get(i).unwrap() == '^' {
                if beam {
                    count += 1;
                    beam_columns.remove(&i);
                    if i > 0 {
                        let prev = chars.get(i - 1).unwrap();
                        if *prev != '^' {
                            beam_columns.insert(i - 1, true);
                        }
                    }
                    if i < chars.len() - 1 {
                        let next = chars.get(i + 1).unwrap();
                        if *next != '^' {
                            beam_columns.insert(i + 1, true);
                        }
                    }
                }
            }
        }

        for i in 0..chars.len() {
            if let Some(_) = beam_columns.get(&i) {
                print!("|");
            } else {
                print!("{}", chars.get(i).unwrap());
            }
        }
        println!();
    }

    println!("Count: {}", count);
}
