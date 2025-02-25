use std::fs;

fn main() {
    let path = "input.in";
    let content = fs::read_to_string(path).expect("Fallo al leer el archivo");

    let mut output: u32 = 0;
    let content: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse().expect("Fallo al parsear"))
                .collect()
        })
        .collect();

    for line in content {
        if is_valid(line.clone(), line[0] > line[1]) {
            output += 1;
        }
    }

    println!("{}", output);
}

fn is_valid(vector: Vec<i32>, descending: bool) -> bool {
    if descending {
        for num in 1..vector.len() {
            if vector[num - 1] - vector[num] < 1 || vector[num - 1] - vector[num] > 3 {
                return problem_dampener(vector);
            }
        }
    } else {
        for num in 1..vector.len() {
            if vector[num - 1] - vector[num] > -1 || vector[num - 1] - vector[num] < -3 {
                return problem_dampener(vector);
            }
        }
    }
    true
}

fn problem_dampener(vector: Vec<i32>) -> bool {
    for num_to_skip in 0..vector.len() {
        let mut vector_attempt: Vec<i32> = Vec::new();
        for num in 0..vector.len() {
            if num != num_to_skip {
                vector_attempt.push(vector[num]);
            }
        }
        let mut valid = true;
        for num in 1..vector_attempt.len() {
            if vector_attempt[0] > vector_attempt[1] {
                if vector_attempt[num - 1] - vector_attempt[num] < 1
                    || vector_attempt[num - 1] - vector_attempt[num] > 3
                {
                    valid = false;
                }
            } else if vector_attempt[0] < vector_attempt[1] {
                if vector_attempt[num - 1] - vector_attempt[num] > -1
                    || vector_attempt[num - 1] - vector_attempt[num] < -3
                {
                    valid = false;
                }
            } else {
                valid = false;
            }
        }
        if valid {
            return true;
        }
    }
    false
}
