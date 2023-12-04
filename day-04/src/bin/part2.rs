use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_card(input: &str) -> Option<&str> {
    let mut temp = input.split(": ");
    temp.next();
    temp.next()
}

fn part2(input: &str) -> String {
    let mut total = 0;

    let mut extra_cards = HashMap::new();
    for (i, line) in input.split("\n").enumerate() {
        let card = match get_card(line) {
            Some(x) => x,
            None => "",
        };
        if card == "" {
            continue;
        }

        let mut halves = card.split(" | ");
        let winning_numbers = match halves.next() {
            Some(x) => x,
            None => "",
        };
        let card_numbers = match halves.next() {
            Some(x) => x,
            None => "",
        };

        let win_lookup: HashMap<usize, &str> = winning_numbers
            .split_whitespace()
            .map(|x| (x.parse::<usize>().unwrap(), x))
            .collect();

        let num_matches = card_numbers
            .split_whitespace()
            .map(|x| (x.parse::<usize>().unwrap()))
            .fold(0, |a, b| {
                if win_lookup.contains_key(&b) {
                    a + 1
                } else {
                    a
                }
            });

        let total_cards = match extra_cards.get(&i) {
            Some(x) => x + 1,
            None => 1,
        };

        total += total_cards;

        for j in i + 1..=i + num_matches {
            extra_cards
                .entry(j)
                .and_modify(|counter| *counter += total_cards)
                .or_insert(total_cards);
        }
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
",
        );
        assert_eq!(result, "30".to_string());
    }
}
