use std::fs;

fn main() {
    let file = fs::read_to_string("day04.dat").unwrap();
    let mut data: Vec<Vec<String>> = vec![];
    for record in file.split("\n") {
        if record.len() == 0 {
            continue;
        }
        data.push(record.chars().map(|c| c.to_string()).collect());
    }
    let row_count = data.len();
    let col_count = data[0].len();
    let mut accessable = 0;
    for i in 0..row_count {
        for j in 0..col_count {
            let start;
            if i == 0 {
                start = 0;
            } else {
                start = i - 1
            }
            let end;
            if i < row_count - 1 {
                end = i + 1;
            } else {
                end = row_count - 1;
            }
            if data[i][j] == "@".to_string() {
                let top;
                if j == 0 {
                    top = 0;
                } else {
                    top = j - 1;
                }
                let bottom;
                if j < col_count - 1 {
                    bottom = j + 1;
                } else {
                    bottom = col_count - 1;
                }
                let mut block_count = 0;
                for x in start..=end {
                    for y in top..=bottom {
                        if data[x][y] == "@".to_string() {
                            block_count += 1;
                        }
                    }
                }
                if block_count < 5 {
                    accessable += 1;
                    print!("X");
                } else {
                    print!("{}", data[i][j]);
                }
            } else {
                print!("{}", data[i][j]);
            }
        }
        println!();
    }
    println!("total accessable: {}", accessable);
}
