use std::fs::File;
use std::io::prelude::*;

fn parse_input(commands: String) -> Vec<usize> {
    commands
        .split(",")
        .filter_map(|v| match v.parse::<usize>() {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .collect::<Vec<usize>>()
}

fn run_int_code(int_code: &mut Vec<usize>) {
    let mut pos: usize = 0;
    loop {
        match int_code[pos] {
            99 => break,
            n => {
                let op1_pos = int_code[pos + 1];
                let op2_pos = int_code[pos + 2];
                let result_pos = int_code[pos + 3];
                match n {
                    1 => int_code[result_pos] = int_code[op1_pos] + int_code[op2_pos],
                    2 => int_code[result_pos] = int_code[op1_pos] * int_code[op2_pos],
                    _ => panic!("Unknown opcode"),
                }
            }
        }
        pos = pos + 4;
    }
}

fn save_int_code(file: &str, int_code: Vec<usize>) {
    let output = int_code
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",");
    std::fs::write(file, output).expect("Could not open output file");
}

fn main() {
    let mut file = File::open("data/day2/input.txt").expect("Unable to open the input file.");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read the file content.");
    let content = content;

    let mut int_code = parse_input(content);

    // Initial correction
    int_code[1] = 12;
    int_code[2] = 2;

    run_int_code(&mut int_code);

    println!("Value at position 0: {}", int_code[0]);

    save_int_code("data/day2/output.txt", int_code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = String::from("1,2,3,4");
        let expected: Vec<usize> = vec![1, 2, 3, 4];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_run_int_code() {
        let mut test_data: (Vec<usize>, Vec<usize>) = (
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        run_int_code(&mut test_data.0);
        assert_eq!(test_data.0, test_data.1);

        let mut test_data: (Vec<usize>, Vec<usize>) = (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        run_int_code(&mut test_data.0);
        assert_eq!(test_data.0, test_data.1);

        let mut test_data: (Vec<usize>, Vec<usize>) = (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        run_int_code(&mut test_data.0);
        assert_eq!(test_data.0, test_data.1);

        let mut test_data: (Vec<usize>, Vec<usize>) =
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        run_int_code(&mut test_data.0);
        assert_eq!(test_data.0, test_data.1);

        let mut test_data: (Vec<usize>, Vec<usize>) = (
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
        run_int_code(&mut test_data.0);
        assert_eq!(test_data.0, test_data.1);
    }
}
