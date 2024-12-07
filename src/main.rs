use day_3::info::print_system_info;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Advent of Code 2024, Day X");

    let start1 = std::time::Instant::now();
    let solution1 = day_3::solve_normal();
    let duration1 = start1.elapsed();

    println!(
        "NORMAL: Solution: {:?}, Duration: {:?}",
        solution1, duration1
    );

    print_system_info();
    Ok(())
}
