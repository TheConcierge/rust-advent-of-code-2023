use nom::{
    bytes::complete::take_until1,
    character::complete::{newline, space1},
    multi::many1,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Rankings {
    five_of_kind: Vec<Player>,
    four_of_kind: Vec<Player>,
    full_house: Vec<Player>,
    three_of_kind: Vec<Player>,
    two_pair: Vec<Player>,
    one_pair: Vec<Player>,
    straight_garbo: Vec<Player>,
}

#[derive(Debug)]
struct Player {
    hand: String,
    bid: u32,
}

fn get_hand(input: &str) -> IResult<&str, Player> {
    let (input, player) = take_until1("\n")(input)?;
    let (temp, hand) = take_until1(" ")(player)?;
    let (temp, _) = space1(temp)?;
    // let (_, bid) = take_until1("\n")(temp)?;
    let bid = temp.parse::<u32>().unwrap();

    let (input, _) = newline(input)?;

    Ok((
        input,
        Player {
            hand: hand.to_string(),
            bid,
        },
    ))
}

fn get_hands(input: &str) -> IResult<&str, Vec<Player>> {
    let (input, hands) = many1(get_hand)(input)?;

    Ok((input, hands))
}

fn part1(input: &str) -> String {
    let total = 0;

    let (_, hands) = get_hands(input).unwrap();
    dbg!(hands);

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
",
        );
        assert_eq!(result, "288".to_string());
    }
}
