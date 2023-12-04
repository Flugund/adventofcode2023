use std::{collections::HashMap, fs};

fn main() {
    let file_path = "./data/input.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file"); // Add the full input data here

    let mut games = HashMap::new();
    parse_input(&input, &mut games);

    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let possible_games_sum = calculate_possible_games(&games, &max_cubes);
    println!("Sum of the IDs of possible games: {}", possible_games_sum);
}

fn parse_input(input: &str, games: &mut HashMap<i32, Vec<(String, i32)>>) {
    for line in input.split('\n') {
        println!("line: {}", line);

        if let Some((id_str, data)) = line.split_once(": ") {
            let id = id_str
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();

            println!("id: {}", id);

            let mut pulls = Vec::new();

            for pull in data.split(';') {
                let mut parts = pull.trim().split_whitespace();
                while let (Some(count), Some(color)) = (parts.next(), parts.next()) {
                    pulls.push((
                        color.trim_matches(',').trim().to_string(),
                        count.trim_matches(',').trim().parse::<i32>().unwrap(),
                    ));
                    println!(
                        "pulls: {} {}",
                        color.trim_matches(',').trim().to_string(),
                        count.trim_matches(',').trim().parse::<i32>().unwrap()
                    );
                }
            }

            games.insert(id, pulls);
        }
    }
}

fn calculate_possible_games(
    games: &HashMap<i32, Vec<(String, i32)>>,
    max_cubes: &HashMap<&str, i32>,
) -> i32 {
    games
        .iter()
        .filter(|(_, pulls)| {
            let mut current_cubes = HashMap::new();

            for (color, count) in *pulls {
                *current_cubes.entry(color).or_insert(0) =
                    i32::max(*current_cubes.get(color).unwrap_or(&0), *count);
            }

            current_cubes
                .iter()
                .all(|(color, &count)| *max_cubes.get(&color[..]).unwrap_or(&0) >= count)
        })
        .map(|(&id, _)| id)
        .sum()
}
