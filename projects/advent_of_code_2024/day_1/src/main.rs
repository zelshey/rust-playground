use std::fs;

fn parse_contents(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let mut group1 = Vec::new();
    let mut group2 = Vec::new();
    for (i, line) in contents.split_whitespace().enumerate() {
        let number: i32 = line.parse().expect("Failed to parse number");
        if i % 2 == 0 {
            group1.push(number);
        } else {
            group2.push(number);
        }
    }
    (group1, group2)
}

fn calculate_distances(group1: &Vec<i32>, group2: &Vec<i32>) -> i32 {
    let mut total_distance = 0;

    for (index, value1) in group1.iter().enumerate() {
        let value2 = group2[index];
        total_distance += (value1 - value2).abs();
    }

    total_distance
}

fn calculate_similarity(group1: &Vec<i32>, group2: &Vec<i32>) -> i32 {
    let mut total_similarity = 0;

    for value1 in group1 {
        let mut simularity = 0;
        for value2 in group2 {
            if value1 == value2 {
                simularity += 1;
            }
        }
        total_similarity += simularity * value1;
    }

    total_similarity
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    let (mut group1, mut group2) = parse_contents(&contents);

    group1.sort();
    group2.sort();

    let total_distance = calculate_distances(&group1, &group2);
    println!("Total Distance: {}", total_distance);

    let total_similarity = calculate_similarity(&group1, &group2);
    println!("Total Similarity: {}", total_similarity);
}
