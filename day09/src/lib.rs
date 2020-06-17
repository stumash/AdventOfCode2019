mod opcode;
use opcode::*;

#[cfg(test)]
use colored::*;

pub fn run_program(input: Vec<i64>, mut data: Vec<i64>) -> Vec<i64> {
    let mut relative_base = 0;
    let mut program_counter = 0;
    let mut output = Vec::new();
    let mut input_iter = input.into_iter();

    while (program_counter as usize) < data.len() {
        let opcode = OpCode::from(CurrentState {
            data: &mut data,
            program_counter: program_counter,
            relative_base: relative_base,
        });
        #[cfg(test)]
        {
            println!("data {}", nums_to_string(&data, Some(program_counter)));
            println!("code {:?}", opcode);
        }
        if let Some(o) = opcode.executeIntruction(
            &mut relative_base,
            &mut data,
            &mut program_counter,
            &mut input_iter,
        ) {
            output.push(o);
        }
        #[cfg(test)]
        {
            println!("out  {}", nums_to_string(&output, None));
        }
    }

    output
}

#[cfg(test)]
fn nums_to_string(nums: &Vec<i64>, color_i: Option<i64>) -> String {
    let ss: Vec<String> = nums
        .iter()
        .enumerate()
        .map(|(i, n)| match color_i {
            Some(color_i) if color_i as usize == i => n.to_string().blue().to_string(),
            _ => n.to_string(),
        })
        .collect();
    ss.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let data = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let output = run_program(vec![], data.clone());
        assert_eq!(output, data);
    }
    #[test]
    fn test_2() {
        let output = run_program(vec![], vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!((*output.get(0).unwrap() as f64).log10().ceil() as i64, 16);
    }
    #[test]
    fn test_3() {
        let data = vec![104, 1125899906842624, 99];
        let output = run_program(vec![], data.clone());
        assert_eq!(data.get(1).unwrap(), output.get(0).unwrap());
    }
    #[test]
    fn test_4() {
        let data = vec![1101, 1, 1, 7, 4, 7, 99, 0];
        let output = run_program(vec![], data.clone());
        assert_eq!(*output.get(0).unwrap(), 2);
    }
    #[test]
    fn test_5() {
        let data = vec![203, 5, 4, 5, 99, 0];
        let input_number = 42;
        let output = run_program(vec![input_number], data.clone());
        assert_eq!(*output.get(0).unwrap(), input_number);
    }
    #[test]
    fn part_1() {
        let data: Vec<i64> = std::fs::read_to_string("data/part1.txt")
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        let output = run_program(vec![1], data);
        assert_eq!(*output.get(0).unwrap(), 3507134798);
    }
}
