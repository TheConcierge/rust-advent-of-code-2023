use itertools::join;
use nom::{
    bytes::complete::take_until1,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn get_nums(input: &str) -> IResult<&str, u64> {
    let (input, line) = take_until1("\n")(input)?;

    let (line, _) = take_until1(" ")(line)?;

    let line = line.trim();

    let (_, nums) = separated_list1(space1, complete::u64)(line)?;

    let (input, _) = newline(input)?;

    let final_num = join(nums, "").parse::<u64>().unwrap();

    Ok((input, final_num))
}

fn get_race_stuff(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, times) = get_nums(input).unwrap();

    let (input, distances) = get_nums(input).unwrap();

    Ok((input, (times, distances)))
}

fn num_wins(race_time: u64, winning_distance: u64) -> u64 {
    let mut total = 0;

    for i in 0..race_time {
        let seconds_held = race_time - i;
        let distance = seconds_held * (race_time - seconds_held);
        if distance > winning_distance {
            total += 1;
        }
    }

    total
}

fn part2(input: &str) -> String {
    let (_, (time, distance)) = get_race_stuff(input).unwrap();

    num_wins(time, distance).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200
",
        );
        assert_eq!(result, "71503".to_string());
    }
}
