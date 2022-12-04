
use std::env;
mod day2;
mod day3;
mod day4;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    day4::solution();
}
