use std::{fs, path::Path};

fn part1(file: &String) {
    fn score_table(x: &str) -> usize {
        match x {
            "AX" => 3,
            "AY" => 6,
            "AZ" => 0,
            "BX" => 0,
            "BY" => 3,
            "BZ" => 6,
            "CX" => 6,
            "CY" => 0,
            "CZ" => 3,

            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        }
    }

    fn score_calculation(opp_choice: &str, you_choice: &str) -> usize {
        let mut score: usize = 0;

        let matchup: String = format!("{}{}", opp_choice, you_choice);

        score += score_table(&matchup);
        score += score_table(you_choice);

        score
    }

    let turns: Vec<&str> = file.split('\n').collect();
    let mut score: usize = 0;

    for turn in turns {
        if turn.is_empty() {
            continue;
        }
        score += score_calculation(&turn[0..1], &turn[2..3]);
    }

    println!("First score is {}", score);
}

fn part2(file: &String) {
    fn scoring_table(x: &str) -> usize {
        match x {
            "AA" => 3,
            "AB" => 6,
            "AC" => 0,
            "BA" => 0,
            "BB" => 3,
            "BC" => 6,
            "CA" => 6,
            "CB" => 0,
            "CC" => 3,

            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
        }
    }

    fn aim_table(x: &str) -> &str {
        match x {
            "AX" => "C",
            "AY" => "A",
            "AZ" => "B",
            "BX" => "A",
            "BY" => "B",
            "BZ" => "C",
            "CX" => "B",
            "CY" => "C",
            "CZ" => "A",
            _ => "",
        }
    }

    fn score_calculation(opp_choice: &str, you_aim: &str) -> usize {
        let mut score: usize = 0;

        let aim = format!("{}{}", opp_choice, you_aim);
        let your_choice = aim_table(&aim);

        let matchup = format!("{}{}", opp_choice, your_choice);

        score += scoring_table(&matchup);
        score += scoring_table(your_choice);

        score
    }

    let rounds: Vec<&str> = file.split('\n').collect();

    let mut total_score: usize = 0;

    for round in rounds {
        if round.is_empty() {
            continue;
        }

        total_score += score_calculation(&round[0..1], &round[2..3]);
    }

    println!("Second score is {}", total_score);
}

fn main() {
    let file: String =
        fs::read_to_string(Path::new("./src/inputs/input.txt")).expect("can't read the contents");

    part1(&file);
    part2(&file);
}
