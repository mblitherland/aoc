use std::fs;

fn main() {
    let file = fs::read_to_string("day06.dat").unwrap();

    let mut rows = vec![];
    let mut operators = vec![];
    
    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let parameters: Vec<&str> = line.split_whitespace().collect();
        let mut row = vec![];
        for p in parameters {
            let result = p.parse::<u64>();
            match result {
                Ok(n) => {
                    row.push(n);
                }
                Err(_) => {
                    operators.push(p);
                }
            }
        }
        if row.len() > 0 {
            rows.push(row);
        }
    }
    println!("r: {:?}", rows);
    println!("o: {:?}", operators);

    let mut col_values = vec![];
    let mut index = 0;
    for c in operators {
        let mut total = 0;
        if c == "+" {
            for r in &rows {
                total += r[index];
            }
        } else if c == "*" {
            total = 1;
            for r in &rows {
                total *= r[index];
            }
        }
        col_values.push(total);

        index += 1;
    }
    println!("cv: {:?}", col_values);

    let mut sum = 0;
    for v in col_values {
        sum += v;
    }
    println!("Final: {}", sum);
}
