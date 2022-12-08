use std::{fs, path::Path};

pub fn part1(path: &Path) -> usize {
    let file_data = fs::read_to_string(path).expect("can't read the contents");
    let mut calories: Vec<usize> = file_data
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .flat_map(|y| usize::from_str_radix(y, 10))
                .sum::<usize>()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    calories[0]
}

pub fn part2(path: &Path) -> usize {
    let file_data = fs::read_to_string(path).expect("can't read the contents");
    let mut calories: Vec<usize> = file_data
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .flat_map(|y| usize::from_str_radix(y, 10))
                .sum::<usize>()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    calories.iter().take(3).sum()
}
