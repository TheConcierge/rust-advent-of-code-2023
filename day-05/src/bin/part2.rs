use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{self, newline},
    multi::{many1, separated_list0},
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug)]
struct Transform {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<Range<u64>>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, temp) = take_until1("\n")(input)?;

    let (_, pairs) = many1(terminated(
        separated_pair(complete::u64, tag(" "), complete::u64),
        tag(" "),
    ))(temp)?;

    let seeds = pairs.iter().map(|x| x.0..x.0 + x.1).collect();

    let (input, _) = newline(input)?;
    let (input, _) = newline(input)?;

    Ok((input, seeds))
}

fn split_range(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, range) = take_until1("\n")(input)?;
    let range = range.trim();
    let (_, range) = separated_list0(tag(" "), complete::u64)(range)?;
    let (input, _) = newline(input)?;
    Ok((input, range))
}

fn get_map(input: &str) -> IResult<&str, Transform> {
    let (input, ranges) = many1(split_range)(input)?;

    let mut res = Transform {
        mappings: Vec::new(),
    };

    for range in ranges {
        res.mappings.push((
            range[1]..(range[1] + range[2]),
            range[0]..(range[0] + range[2]),
        ));
    }

    return Ok((input, res));
}

fn parse_map(input: &str) -> IResult<&str, Transform> {
    let (input, _) = take_until1("map:")(input)?;
    let (input, _) = tag("map:")(input)?;
    let (input, _) = newline(input)?;
    let (input, map) = get_map(input)?;
    Ok((input, map))
}

fn lets_try(pos: &u64, source_start: &u64, dest_start: &u64) -> u64 {
    let offset = pos - source_start;
    return dest_start + offset;
}

fn part2(input: &str) -> String {
    let (input, seeds) = parse_seeds(input).unwrap();

    let (_, blocks) = many1(parse_map)(input).unwrap();

    let closest_location = seeds
        .into_par_iter()
        .flat_map(|x| x.clone())
        .map(|n| {
            blocks.iter().fold(n, |y, z| {
                z.mappings.iter().fold(y, |q, t| {
                    if q != y {
                        q
                    } else {
                        if t.0.contains(&q) {
                            let temp = lets_try(&q, &t.0.start, &t.1.start);
                            temp
                        } else {
                            q
                        }
                    }
                })
            })
        })
        .min();

    match closest_location {
        Some(x) => x.to_string(),
        None => String::from("could not find min"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part2(
            "seeds: 79 14 55 13 

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

",
        );
        assert_eq!(result, "46".to_string());
    }
}
