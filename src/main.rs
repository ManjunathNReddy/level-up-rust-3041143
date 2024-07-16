mod median;

fn run_median() {
    println!("01 Median: Enter the numbers");
    let median = median::run();
    match median {
        Some(value) => println!("Median is {}", value),
        None => println!("No median for empty list!"),
    }
}
fn main() {
    run_median();
}
