fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut total = 0;

    let bytes = input.as_bytes();
    let mut found = vec![];
    for (_, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            if found.len() == 0 {
                continue;
            }

            if found.len() == 1 {
                found.push(found[0])
            }

            let temp = String::from_utf8(found).unwrap();
            total += temp.parse::<i32>().unwrap();
            found = vec![];
        }
        if item.is_ascii_digit() {
            if found.len() > 1 {
                found[1] = item;
            } else {
                found.push(item);
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
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );
        assert_eq!(result, "142".to_string());
    }
}
