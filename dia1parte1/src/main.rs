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

    left_vec.sort();
    right_vec.sort();
    let mut output: u32 = 0;

    for index in 0..left_vec.len() {
        let num;
        if left_vec[index] > right_vec[index] {
            num = left_vec[index] - right_vec[index];
        } else {
            num = right_vec[index] - left_vec[index];
        }
        output += num;
    }

    println!("{}", output)
}
