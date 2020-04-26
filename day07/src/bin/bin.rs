use colored::*;
use day07::OpCode;
use itertools::Itertools;
use core::iter;

fn main() {
    let data = vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 59, 84, 93, 110, 191, 272, 353, 434, 99999, 3,
        9, 101, 2, 9, 9, 102, 3, 9, 9, 1001, 9, 5, 9, 102, 4, 9, 9, 1001, 9, 4, 9, 4, 9, 99, 3, 9,
        101, 3, 9, 9, 102, 5, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 4, 9, 1002, 9, 2, 9, 101, 2, 9, 9,
        102, 2, 9, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9,
        1001, 9, 5, 9, 1002, 9, 3, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2,
        9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9,
        9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9,
        3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        101, 1, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
        9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101,
        1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
        3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001,
        9, 2, 9, 4, 9, 99,
    ];

    let phase_settings: Vec<i32> = (5..=9).collect();
    let mut max = 0;
    for phase_setting in phase_settings.iter().permutations(phase_settings.len()) {

        let mut stacks: Vec<ProgramState> = iter::repeat(ProgramState::new(&data)).take(phase_settings.len()).collect();
        let mut output = None;

        output = phase_setting.iter().zip(stacks.iter_mut()).fold(Some(0), |input, (phase, stack)| {
            match input {
                None => None,
                Some(input) => run_partial_program(stack, &mut vec![*phase.clone(), input].into_iter())
            }
        });

        let mut last_output = output;
        while let Some(_) = last_output {
            output = last_output;
            last_output = stacks.iter_mut().fold(last_output, |lo, stack| {
                match lo {
                    None => None,
                    Some(lo) => run_partial_program(stack, &mut vec![lo].into_iter())
                }
            });
        }

        if let Some(o) = output {
            if o > max {
                max = o;
            }
        }
    }
    println!("max: {}", max);
}

#[derive(Clone)]
struct ProgramState {
    programCounter: i32,
    data: Vec<i32>,
}
impl ProgramState {
    fn new(data: &Vec<i32>) -> Self {
        ProgramState {
            programCounter: 0,
            data: data.clone()
        }
    }
}

fn run_partial_program(
    stack: &mut ProgramState,
    input: &mut dyn Iterator<Item = i32>,
) -> Option<i32> {
    let mut count = 0;
    let mut i = &mut stack.programCounter;
    let mut data = &mut stack.data;
    while 0 <= *i && (*i as usize) < data.len() {
        //println!("-----------------------\ncount: {}, i: {:?}\ndata: {}\n", count, i, nums_to_string(data, Some(i)));
        let opcode = OpCode::from((&*data, *i));
        if let Some(val) = opcode.executeIntruction(data, &mut i, input) {
            return Some(val);
        }
        //println!("opcode {:?}\ndata: {}", opcode, nums_to_string(data, Some(i)));
        count += 1;
    }

    None
}

fn runProgram(data: &mut Vec<i32>, input: &mut dyn Iterator<Item = i32>) -> Vec<i32> {
    let mut output = Vec::new();

    let mut count = 0;
    let mut i: i32 = 0;
    while 0 <= i && (i as usize) < data.len() {
        //println!("-----------------------\ncount: {}, i: {:?}\ndata: {}\n", count, i, nums_to_string(data, Some(i)));
        let opcode = OpCode::from((&*data, i));
        if let Some(val) = opcode.executeIntruction(data, &mut i, input) {
            output.push(val);
        }
        //println!("opcode {:?}\ndata: {}", opcode, nums_to_string(data, Some(i)));
        count += 1;
    }

    output
}

fn nums_to_string(nums: &Vec<i32>, color_i: Option<i32>) -> String {
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
    use super::runProgram;
    use std::cmp::Ordering;

    #[test]
    fn day05_basic1() {
        for input_i in 1..10 {
            let mut data = vec![3, 0, 4, 0, 99];
            let input = vec![input_i.clone()];
            assert_eq!(vec![input_i], runProgram(&mut data, &mut input.into_iter()))
        }
    }

    #[test]
    fn day05_basic2() {
        for input_i in 6..=10 {
            let mut data = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![if input_i == 8 { 1 } else { 0 }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05_basic3() {
        for input_i in 6..=10 {
            let mut data = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![if input_i < 8 { 1 } else { 0 }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05_basic4() {
        for input_i in 6..=11 {
            let mut data = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![if input_i == 8 { 1 } else { 0 }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05_basic5() {
        for input_i in 6..=10 {
            let mut data = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![if input_i < 8 { 1 } else { 0 }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05_basic6() {
        for input_i in 0..=1 {
            let mut data = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![if input_i == 0 { 0 } else { 1 }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05_basic7() {
        for input_i in 0..=1 {
            let mut data = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![if input_i == 0 { 0 } else { 1 }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05_basic8() {
        for input_i in 6..=10 {
            println!("\n\ninput_i: {}", input_i);
            let mut data = vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ];
            let input = vec![input_i.clone()];
            assert_eq!(
                vec![match input_i.cmp(&8) {
                    Ordering::Less => 999,
                    Ordering::Equal => 1000,
                    Ordering::Greater => 1001,
                }],
                runProgram(&mut data, &mut input.into_iter())
            );
        }
    }

    #[test]
    fn day05() {
        let mut data = vec![
            3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 40, 93, 224, 1001, 224, -3720,
            224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224, 223, 223, 1101, 56, 23, 225,
            1102, 64, 78, 225, 1102, 14, 11, 225, 1101, 84, 27, 225, 1101, 7, 82, 224, 1001, 224,
            -89, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 1, 224, 1, 224, 223, 223, 1, 35, 47,
            224, 1001, 224, -140, 224, 4, 224, 1002, 223, 8, 223, 101, 5, 224, 224, 1, 224, 223,
            223, 1101, 75, 90, 225, 101, 9, 122, 224, 101, -72, 224, 224, 4, 224, 1002, 223, 8,
            223, 101, 6, 224, 224, 1, 224, 223, 223, 1102, 36, 63, 225, 1002, 192, 29, 224, 1001,
            224, -1218, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 223, 224, 223, 102,
            31, 218, 224, 101, -2046, 224, 224, 4, 224, 102, 8, 223, 223, 101, 4, 224, 224, 1, 224,
            223, 223, 1001, 43, 38, 224, 101, -52, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 5,
            224, 224, 1, 223, 224, 223, 1102, 33, 42, 225, 2, 95, 40, 224, 101, -5850, 224, 224, 4,
            224, 1002, 223, 8, 223, 1001, 224, 7, 224, 1, 224, 223, 223, 1102, 37, 66, 225, 4, 223,
            99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247,
            1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106,
            0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280,
            1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0,
            300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999,
            1007, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 329, 1001, 223, 1, 223, 1007, 226,
            226, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 1107, 677, 226, 224,
            102, 2, 223, 223, 1006, 224, 359, 1001, 223, 1, 223, 108, 677, 677, 224, 1002, 223, 2,
            223, 1006, 224, 374, 1001, 223, 1, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005,
            224, 389, 101, 1, 223, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 1001,
            223, 1, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 419, 101, 1, 223, 223,
            1008, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 434, 101, 1, 223, 223, 1008, 226,
            226, 224, 1002, 223, 2, 223, 1005, 224, 449, 101, 1, 223, 223, 7, 677, 226, 224, 1002,
            223, 2, 223, 1006, 224, 464, 1001, 223, 1, 223, 7, 226, 226, 224, 1002, 223, 2, 223,
            1005, 224, 479, 1001, 223, 1, 223, 1007, 677, 677, 224, 102, 2, 223, 223, 1005, 224,
            494, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 509, 1001,
            223, 1, 223, 8, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 524, 1001, 223, 1, 223,
            1107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 539, 1001, 223, 1, 223, 1008, 226,
            677, 224, 1002, 223, 2, 223, 1006, 224, 554, 1001, 223, 1, 223, 1107, 226, 677, 224,
            1002, 223, 2, 223, 1006, 224, 569, 1001, 223, 1, 223, 1108, 677, 677, 224, 102, 2, 223,
            223, 1005, 224, 584, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1006, 224,
            599, 1001, 223, 1, 223, 1108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 614, 101, 1,
            223, 223, 107, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223, 223, 108,
            226, 677, 224, 1002, 223, 2, 223, 1005, 224, 644, 101, 1, 223, 223, 8, 226, 677, 224,
            1002, 223, 2, 223, 1005, 224, 659, 1001, 223, 1, 223, 107, 226, 226, 224, 1002, 223, 2,
            223, 1006, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226,
        ];
        let input = vec![5];
        assert_eq!(vec![9168267], runProgram(&mut data, &mut input.into_iter()))
    }
}
