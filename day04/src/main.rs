fn main() {
    println!("Hello, world!");
}

fn is_six_digits(u: u32) -> bool {
    u / 10.pow(6) == 0 && u / 10.pow(5) != 0
}
