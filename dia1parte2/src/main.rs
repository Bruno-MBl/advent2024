use std::fs;

fn main() {
    let file_path = "input.in";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut left_vec: Vec<u32> = Vec::new();
    let mut right_vec: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let mut nums = line.split("   ");
        left_vec.push(nums.next().unwrap().parse().unwrap());
        right_vec.push(nums.next().unwrap().parse().unwrap());
    }

    let mut output: u32 = 0;

    for x in 0..left_vec.len() {
        for y in 0..right_vec.len() {
            if left_vec[x] == right_vec[y] {
                output += left_vec[x];
            }
        }
    }

    println!("{}", output)
}
