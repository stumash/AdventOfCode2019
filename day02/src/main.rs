use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let mut nums: Vec<u32> = line
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let desired_program_result = 19690720;
    let mut noun: u32 = 0;
    let mut verb: u32 = 0;

    for n in 0..100 {
        for v in 0..100 {
            nums[1] = n;
            nums[2] = v;

            if run_program(&nums) == desired_program_result {
                noun = n;
                verb = v;
                break;
            }
        }
    }

    println!("{}", 100 * noun + verb);
}

enum OpCode {
    Add,
    Mult,
    Halt
}

impl OpCode {
    fn new(i: u32) -> OpCode {
        match i {
            1 => OpCode::Add,
            2 => OpCode::Mult,
            99 => OpCode::Halt,
            _ => panic!("{} is an invalid opcode, must be in [1, 2, 99]", i)
        }
    }
}

fn print_program(nums: &Vec<u32>) {
    println!("-----------------------------------------------------");
    for i in (0..nums.len()).step_by(4) {
        if i+1 < nums.len() {
            println!("at {} do {} to  {} {} {}", i, nums[i], nums[i+1], nums[i+2], nums[i+3]);
        } else {
            println!("at {} do {} -", i, nums[i]);
        }
    }
}

fn run_program(nums_p: &Vec<u32>) -> u32 {
    let mut nums = nums_p.clone();

    for i in (0..nums.len()-1).step_by(4) {
        match OpCode::new(nums[i]) {
            OpCode::Add => {
                let store_index = nums[i+3];
                nums[store_index as usize] = nums[nums[i+1] as usize] + nums[nums[i+2] as usize];
            },
            OpCode::Mult => {
                let store_index = nums[i+3];
                nums[store_index as usize] = nums[nums[i+1] as usize] * nums[nums[i+2] as usize];
            },
            OpCode::Halt => break,
        }
    }

    nums[0]
}
