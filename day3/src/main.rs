use std::{collections::HashMap, fs, path::Path};

fn part1(file_data: &String) {
    let values: Vec<&str> = file_data.split('\n').collect();

    let mut total: usize = 0;

    for val in values {
        if val.is_empty() {
            continue;
        };

        let pouch1 = &val[0..val.len() / 2];
        let pouch2 = &val[val.len() / 2..val.len()];

        let repeated = repeated_character_find(pouch1, pouch2);

        total += priority_calculation(repeated);
    }

    println!("First part answer {}", total);

    fn repeated_character_find(text1: &str, text2: &str) -> char {
        let mut repeats: HashMap<char, usize> = HashMap::new();

        for char in text1.chars() {
            repeats.entry(char).or_insert(1);
        }

        for char in text2.chars() {
            if repeats.contains_key(&char) {
                return char;
            }
        }

        ' '
    }

    fn priority_calculation(char: char) -> usize {
        let mut priotity: usize;

        if char.is_lowercase() {
            priotity = char.to_uppercase().last().unwrap() as usize;
        } else {
            priotity = char.to_lowercase().last().unwrap() as usize - 6;
        }

        priotity -= 64;
        priotity
    }
}

fn part2(file_data: &String) {
    let mut total_priority: usize = 0;

    let bags: Vec<&str> = file_data.split('\n').collect();

    let mut i = 0;

    while i < bags.len() - 3 {
        let repeated: char = find_repeated(bags[i], bags[i + 1], bags[i + 2]);
        total_priority += calculate_priority(repeated);

        i += 3;
    }

    println!("second part anwser: {}", total_priority);

    fn find_repeated(text1: &str, text2: &str, text3: &str) -> char {
        let mut repeats: HashMap<char, usize> = HashMap::new();

        for char in text1.chars() {
            repeats.entry(char).or_insert(1);
        }

        for char in text2.chars() {
            if repeats.contains_key(&char) {
                repeats.insert(char, 2);
            }
        }

        for char in text3.chars() {
            if repeats.get(&char).copied().unwrap_or(0) == 2 {
                return char;
            }
        }

        ' '
    }

    fn calculate_priority(char: char) -> usize {
        let mut priotity: usize;
        if char.is_lowercase() {
            priotity = char.to_uppercase().last().unwrap() as usize;
        } else {
            priotity = char.to_lowercase().last().unwrap() as usize - 6;
        }

        priotity -= 64;
        priotity
    }
}

fn main() {
    let file: String =
        fs::read_to_string(Path::new("./src/inputs/input.txt")).expect("can't read the contents");
    part1(&file);
    part2(&file);
}
