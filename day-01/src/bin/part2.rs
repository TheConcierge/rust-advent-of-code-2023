fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

struct Combo {
    value: String,
    position: i32,
}

fn more_left(input: &str, dig_val: &str, str_val: &str, mut curr_left: Combo) -> Combo {
    let result = input.find(dig_val);
    curr_left = match result {
        Some(x) => {
            let pos: i32 = x as i32;
            if pos < curr_left.position {
                Combo {
                    value: dig_val.to_string(),
                    position: pos,
                }
            } else {
                curr_left
            }
        }
        None => curr_left,
    };

    let result = input.find(str_val);
    curr_left = match result {
        Some(x) => {
            let pos: i32 = x as i32;
            if pos < curr_left.position {
                Combo {
                    value: dig_val.to_string(),
                    position: pos,
                }
            } else {
                curr_left
            }
        }
        None => curr_left,
    };

    return curr_left;
}

fn more_right(input: &str, dig_val: &str, str_val: &str, mut curr_right: Combo) -> Combo {
    let result = input.rfind(dig_val);
    curr_right = match result {
        Some(x) => {
            let pos: i32 = x as i32;
            if pos > curr_right.position {
                let n = Combo {
                    value: dig_val.to_string(),
                    position: pos,
                };
                n
            } else {
                curr_right
            }
        }
        None => curr_right,
    };

    let result = input.rfind(str_val);

    curr_right = match result {
        Some(x) => {
            let pos: i32 = x as i32;
            if pos > curr_right.position {
                let n = Combo {
                    value: dig_val.to_string(),
                    position: pos,
                };
                n
            } else {
                curr_right
            }
        }
        None => curr_right,
    };

    return curr_right;
}

fn part2(input: &str) -> String {
    let mut total = 0;

    let parts = input.split("\n");
    for part in parts {
        let mut left = Combo {
            value: "-1".to_string(),
            position: part.len() as i32,
        };
        let mut right = Combo {
            value: "-1".to_string(),
            position: -1,
        };

        left = more_left(part, "1", "one", left);
        left = more_left(part, "2", "two", left);
        left = more_left(part, "3", "three", left);
        left = more_left(part, "4", "four", left);
        left = more_left(part, "5", "five", left);
        left = more_left(part, "6", "six", left);
        left = more_left(part, "7", "seven", left);
        left = more_left(part, "8", "eight", left);
        left = more_left(part, "9", "nine", left);

        right = more_right(part, "1", "one", right);
        right = more_right(part, "2", "two", right);
        right = more_right(part, "3", "three", right);
        right = more_right(part, "4", "four", right);
        right = more_right(part, "5", "five", right);
        right = more_right(part, "6", "six", right);
        right = more_right(part, "7", "seven", right);
        right = more_right(part, "8", "eight", right);
        right = more_right(part, "9", "nine", right);

        let found = format!("{}{}", left.value, right.value);
        println!("{}", found);
        let result = found.parse::<i32>();
        total = match result {
            Ok(x) => total + x,
            Err(_) => total,
        };
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }
}
