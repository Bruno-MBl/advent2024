use std::{char, collections::VecDeque, fs};

fn main() {
    let file_path = "input.in";
    let contents = fs::read_to_string(file_path).expect("Error al leer el archivo");
    let mut contents = contents.chars();
    let mut vec: VecDeque<char> = VecDeque::new();

    for _ in 0..11 {
        vec.push_back(contents.next().unwrap());
    }

    let mut acc: u32 = 0;
    vec.push_front('0');

    while let Some(char) = contents.next() {
        vec.pop_front();
        vec.push_back(char);

        if vec[0] != 'm' {
            continue;
        }
        if vec[1] != 'u' {
            continue;
        }
        if vec[2] != 'l' {
            continue;
        }
        if vec[3] != '(' {
            continue;
        }
        let slice = vec.make_contiguous();
        let pos = slice.iter().position(|char| *char == ')');
        let pos = match pos {
            Some(val) => val,
            None => {
                continue;
            }
        };
        acc += calc(&&slice[4..pos]);
    }
    println!("{acc}");
}
fn calc(nums: &[char]) -> u32 {
    let mut iter = nums.split(|char| *char == ',');
    if iter.clone().count() != 2 {
        return 0;
    }
    let num1: String = iter.next().unwrap_or(&['0']).iter().collect();
    let num2: String = iter.next().unwrap_or(&['0']).iter().collect();
    if let Ok(num1) = num1.parse::<u32>() {
        if let Ok(num2) = num2.parse::<u32>() {
            return num1 * num2;
        }
    }
    return 0;
}
