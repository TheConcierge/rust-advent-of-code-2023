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

fn part1(input: &str) -> String {
    let the_bag = CubeCounter {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut total = 0;

    let games = input.split("\n");
    let mut game_num = 0;
    for game in games {
        game_num += 1;
        let mut valid = true;

        let temp: Vec<&str> = game.split(": ").collect();
        if temp.len() < 2 {
            continue;
        }
        let rounds = temp[1].split("; ");
        for round in rounds {
            for draw in round.split(", ") {
                let draw_breakdown: Vec<&str> = draw.split(" ").collect();
                let color = draw_breakdown[1];
                let amount = draw_breakdown[0].parse::<i32>().unwrap();
                if color == "blue" && amount > the_bag.blue {
                    valid = false;
                } else if color == "red" && amount > the_bag.red {
                    valid = false;
                } else if color == "green" && amount > the_bag.green {
                    valid = false;
                }
            }
        }

        if !valid {
            continue;
        }

        total = total + game_num;
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
        assert_eq!(result, "8".to_string());
    }
}
