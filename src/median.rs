use std::io;

pub fn run() -> Option<f32> {
    let res;
    let mut input = String::new();
    // Read input from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut nums: Vec<f32> = input
        .trim() // Remove any trailing newline
        .split_whitespace() // Split by whitespace
        .map(|s| s.parse().expect("Please enter valid numbers")) // Parse each substring to f32
        .collect();
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let size = nums.len();
    if size > 0 {
        let mid_index = size / 2;
        // Calculate median according to parity
        if size % 2 == 0 {
            res = (nums[mid_index - 1] + nums[mid_index]) / 2.0;
        } else {
            res = nums[size / 2];
        }
    } else {
        return None;
    }
    Some(res)
}
