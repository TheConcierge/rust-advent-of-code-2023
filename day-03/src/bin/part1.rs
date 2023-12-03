use trie_rs::Trie;
use trie_rs::TrieBuilder;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_symbol_coords(input: &str) -> Trie<String> {
    let mut builder = TrieBuilder::new();
    let lines = input.split("\n");
    for (i, line) in lines.enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            if ch.is_ascii_digit() {
                continue;
            }

            let coord_pair = vec![i.to_string(), j.to_string()];
            builder.push(coord_pair);
        }
    }
    let coord_trie = builder.build();

    return coord_trie;
}

fn get_adjacent_coords(i: i32, j: i32, num_length: i32) -> Vec<(i32, i32)> {
    let mut adjacent_coords = vec![];

    adjacent_coords.push((i, j - 1));
    adjacent_coords.push((i + 1, j - 1));
    adjacent_coords.push((i - 1, j - 1));

    for n in j..j + num_length + 1 {
        adjacent_coords.push((i + 1, n as i32));
        adjacent_coords.push((i - 1, n as i32));
    }

    adjacent_coords.push((i, j + num_length));
    adjacent_coords.push((i + 1, j + num_length));
    adjacent_coords.push((i - 1, j + num_length));

    return adjacent_coords;
}

fn has_adjacent_symbol(coords: &Vec<(i32, i32)>, symbol_tracker: &Trie<String>) -> bool {
    for coord in coords {
        if symbol_tracker.exact_match(vec![coord.0.to_string(), coord.1.to_string()]) {
            return true;
        }
    }

    return false;
}

fn part1(input: &str) -> String {
    let mut total = 0;

    let symbol_coords = get_symbol_coords(input);

    for (i, line) in input.split("\n").enumerate() {
        let mut curr_num = String::from("");
        // add an extra "." to the end of every line. this should solve the issue of a
        // number at the end of a line.
        let adjusted_line = format!("{}{}", line, ".");
        for (j, ch) in adjusted_line.chars().enumerate() {
            if ch.is_ascii_digit() {
                curr_num.push_str(&format!("{}", ch));
                continue;
            }

            // found a non-digit character, process number if doesn't exist
            if curr_num == "" {
                continue;
            }

            // we are at the end of the number, pass in starting position instead
            let starting_j = j - curr_num.len();

            let adjacent_coords =
                get_adjacent_coords(i as i32, starting_j as i32, curr_num.len() as i32);

            if has_adjacent_symbol(&adjacent_coords, &symbol_coords) {
                total += curr_num.parse::<i32>().unwrap();
            }

            curr_num = String::from("");
        }
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
",
        );
        assert_eq!(result, "4361".to_string());
    }
}
