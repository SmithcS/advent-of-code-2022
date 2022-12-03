use std::fs;
use std::path::Path;
use std::io::{self, BufRead};

const INPUT_FILE_PATH_ROOT: &str = "/Users/smithsopp/Documents/programming/advent-of-code-2022/src/input_files/";

pub fn read_input_file(input_file_name: &str) -> String {
    let file_path = format!("{INPUT_FILE_PATH_ROOT}/{input_file_name}");
    return fs::read_to_string(file_path).expect("Unable to read the file");
}

pub fn read_input_file_into_vector(input_file_name: &str) -> Vec<Vec<i32>> {
    let file_path = format!("{INPUT_FILE_PATH_ROOT}/{input_file_name}");
    let mut file_lines = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        let mut row = Vec::new();

        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    file_lines.push(row);
                    row = Vec::new();
                    continue;
                }

                let str_as_int = ip.parse::<i32>().unwrap();
                row.push(str_as_int);
            }
        }
    }

    file_lines
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}