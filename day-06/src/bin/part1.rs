use nom::{
    bytes::complete::take_until1,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn get_nums(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, line) = take_until1("\n")(input)?;

    let (line, _) = take_until1(" ")(line)?;

    let line = line.trim();

    let (_, nums) = separated_list1(space1, complete::u64)(line)?;

    let (input, _) = newline(input)?;

    Ok((input, nums))
}

fn get_race_stuff(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    let (input, times) = get_nums(input).unwrap();

    let (input, distances) = get_nums(input).unwrap();

    Ok((input, (times, distances)))
}

fn num_wins(race_info: &(u64, u64)) -> u64 {
    let mut total = 0;

    let race_time = race_info.0;
    let winning_distance = race_info.1;

    for i in 0..race_time {
        let seconds_held = race_time - i;
        let distance = seconds_held * (race_time - seconds_held);
        if distance > winning_distance {
            total += 1;
        }
    }

    total
}

fn part1(input: &str) -> String {
    let (_, (times, distances)) = get_race_stuff(input).unwrap();
    let result = times
        .into_iter()
        .zip(distances)
        .map(|race| num_wins(&race))
        .reduce(|x, y| if y == 0 { x } else { x * y });

    match result {
        Some(x) => x.to_string(),
        None => String::from("could not calculate an answer"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200
",
        );
        assert_eq!(result, "288".to_string());
    }
}
