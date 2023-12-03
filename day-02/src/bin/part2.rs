fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

struct CubeCounter {
    green: i32,
    blue: i32,
    red: i32,
}

fn process_round(round: &str) -> CubeCounter {
    let mut round_counter = CubeCounter {
        green: 0,
        blue: 0,
        red: 0,
    };

    for draw in round.split(", ") {
        let draw_breakdown: Vec<&str> = draw.split(" ").collect();
        let color = draw_breakdown[1];
        let amount = draw_breakdown[0].parse::<i32>().unwrap();

        if color == "blue" {
            round_counter.blue += amount;
        } else if color == "red" {
            round_counter.red += amount;
        } else if color == "green" {
            round_counter.green += amount;
        }
    }

    return round_counter;
}

fn part1(input: &str) -> String {
    let mut total = 0;

    let games = input.split("\n");
    for game in games {
        let temp: Vec<&str> = game.split(": ").collect();
        if temp.len() < 2 {
            continue;
        }

        let mut game_counter = CubeCounter {
            green: 0,
            blue: 0,
            red: 0,
        };

        let rounds = temp[1].split("; ");
        for round in rounds {
            let round_counter = process_round(round);
            game_counter.red = game_counter.red.max(round_counter.red);
            game_counter.blue = game_counter.blue.max(round_counter.blue);
            game_counter.green = game_counter.green.max(round_counter.green);
        }

        total += game_counter.red * game_counter.blue * game_counter.green;
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
",
        );
        assert_eq!(result, "2286".to_string());
    }
}
