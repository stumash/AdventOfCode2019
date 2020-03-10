use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let nums_as_strings: Vec<&str> = line.split("-").collect();
    let (num1_string, num2_string) = (*nums_as_strings.get(0).unwrap(), *nums_as_strings.get(1).unwrap());
    let (num1, num2) = (Number::from(num1_string), Number::from(num2_string));

    let mut count = 0;
    let mut curr = num1.clone();
    while curr != num2 {
        if curr.no_decreasing_digits() && curr.adjacent_repeat_pair() {
            count += 1;
        }
        curr.incr();
    }

    println!("{}", count);
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq, Hash)]
enum Digit {
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine
}
impl Digit {
    fn succ(&self) -> Digit {
        match self {
            Digit::Zero => Digit::One,
            Digit::One => Digit::Two,
            Digit::Two => Digit::Three,
            Digit::Three => Digit::Four,
            Digit::Four => Digit::Five,
            Digit::Five => Digit::Six,
            Digit::Six => Digit::Seven,
            Digit::Seven => Digit::Eight,
            Digit::Eight => Digit::Nine,
            Digit::Nine=> Digit::Zero
        }
    }
}
impl From<char> for Digit {
    fn from(c: char) -> Digit {
        match c {
            '0' => Digit::Zero,
            '1' => Digit::One,
            '2' => Digit::Two,
            '3' => Digit::Three,
            '4' => Digit::Four,
            '5' => Digit::Five,
            '6' => Digit::Six,
            '7' => Digit::Seven,
            '8' => Digit::Eight,
            '9' => Digit::Nine,
            _ => panic!("failed to parse digit from {}", c)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number{
    digits: Vec<Digit>
}
impl Number {
    fn incr(&mut self) {
        for i in (0..self.digits.len()).rev() {
            self.digits[i] = self.digits[i].succ();
            if self.digits[i] != Digit::Zero {
                break
            }
        }
    }
    fn no_decreasing_digits(&self) -> bool {
        for i in 1..self.digits.len() {
            if self.digits[i-1] > self.digits[i] {
                return false
            }
        }
        true
    }
    fn adjacent_repeat_pair(&self) -> bool {
        let counter = self.digits.iter().fold(HashMap::new(), move |mut hm, d| {
            match hm.get(d) {
                None => hm.insert(d, 1),
                Some(i) => hm.insert(d, i+1)
            };
            hm
        });
        for tally in counter.values() {
            if *tally == 2 {
                return true
            }
        }
        false
    }
}
impl From<&str> for Number {
    fn from(s: &str) -> Number {
        Number{
            digits: s.chars().map(|c| Digit::from(c)).collect()
        }
    }
}
