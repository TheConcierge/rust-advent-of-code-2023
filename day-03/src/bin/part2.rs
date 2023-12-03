use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn coords_to_key(row: i32, column: i32) -> String {
    return format!("{},{}", row, column);
}

fn get_gear_ratio(row: i32, column: i32, part_map: &HashMap<String, i32>) -> Option<i32> {
    let mut ratio = 1;
    let mut adjacent_parts = 0;
    // using processed_parts to make sure we don't process the same value if multiple of its coordinates
    // are adjacent. this assumes part numbers are unique :fingers-crossed:
    let mut processed_parts = HashMap::new();

    for i in row - 1..row + 2 {
        for j in column - 1..column + 2 {
            let coord_key = &coords_to_key(i as i32, j as i32);
            if part_map.contains_key(coord_key) {
                let part_id = part_map[coord_key];
                if !processed_parts.contains_key(&part_id) {
                    adjacent_parts += 1;
                    ratio *= part_map[coord_key];
                    processed_parts.insert(part_id, "");
                }
            }
        }
    }

    if adjacent_parts == 2 {
        return Some(ratio);
    }

    return None;
}

fn get_number_coords(input: &str) -> HashMap<String, i32> {
    let mut map = HashMap::new();

    for (row, line) in input.split("\n").enumerate() {
        let mut curr_num = String::from("");
        // add an extra "." to the end of every line. this should solve the issue of a
        // number at the end of a line.
        let adjusted_line = format!("{}{}", line, ".");
        let mut collected_coords: Vec<(usize, usize)> = vec![];
        for (column, ch) in adjusted_line.chars().enumerate() {
            if ch.is_ascii_digit() {
                curr_num.push_str(&format!("{}", ch));
                collected_coords.push((row, column));
                continue;
            }

            // found a non-digit character, process number if doesn't exist
            if curr_num == "" {
                continue;
            }

            let val = curr_num.parse::<i32>().unwrap();
            for coord in collected_coords {
                map.insert(coords_to_key(coord.0 as i32, coord.1 as i32), val);
            }

            curr_num = String::from("");
            collected_coords = vec![];
        }
    }

    return map;
}

fn part2(input: &str) -> String {
    let mut total = 0;

    let part_coords = get_number_coords(input);
    println!("{:#?}", part_coords);

    for (row, line) in input.split("\n").enumerate() {
        // add an extra "." to the end of every line. this should solve the issue of a
        // number at the end of a line.
        let adjusted_line = format!("{}{}", line, ".");
        for (column, ch) in adjusted_line.chars().enumerate() {
            if ch == '*' {
                total = match get_gear_ratio(row as i32, column as i32, &part_coords) {
                    Some(x) => {
                        total = total + x;
                        total
                    }
                    None => total,
                };
            }
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
        assert_eq!(result, "467835".to_string());
    }
}
