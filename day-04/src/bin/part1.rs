use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_card(input: &str) -> Option<&str> {
    let mut temp = input.split(": ");
    temp.next();
    temp.next()
}

fn part1(input: &str) -> String {
    let mut total = 0;

    for line in input.split("\n") {
        let card = match get_card(line) {
            Some(x) => x,
            None => "",
        };
        let mut halves = card.split(" | ");
        let winning_numbers = match halves.next() {
            Some(x) => x,
            None => "",
        };
        let card_numbers = match halves.next() {
            Some(x) => x,
            None => "",
        };

        // makes for easier lookup, plus i get to practice iterators
        let win_lookup: HashMap<i32, &str> = winning_numbers
            .split_whitespace()
            .map(|x| (x.parse::<i32>().unwrap(), x))
            .collect();

        let score = card_numbers
            .split_whitespace()
            .map(|x| (x.parse::<i32>().unwrap()))
            .fold(0, |a, b| {
                if win_lookup.contains_key(&b) {
                    if a == 0 {
                        1
                    } else {
                        a * 2
                    }
                } else {
                    a
                }
            });
        total += score;
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(result, "13".to_string());
    }
}
