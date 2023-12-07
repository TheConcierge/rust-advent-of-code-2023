# Day 4
## Link to puzzle
https://adventofcode.com/2023/day/4
## Part 1
### Problem Definition
We are given a series of scratch cards, defined as such:
```
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```

On the left are winning numbers and the right are our numbers. If a single number
matches a winning number, the ticket is worth 1 point. For each additional matching
number, the point value is multiplied by 2.

The goal is to calculate the total number of points amongst all of the tickets. 

### Approach
This was very straightforward. For each game:
- split the winning numbers from our numbers
- map each winning number into a HashMap entry for lookup
- iterate over our numbers, adjust the game score if number is in the winning number HashMap
- increment total

### Solution
>! 509115

## Part 2
### Problem Defintion
This time, there is no concept of points. For each game, matching a number will win
you a copy of a subsequent card.

So, if you win a copy of card 10 and it has 5 matching numbers, it would then win a copy of the same cards that 
the original card 10 won: cards 11, 12, 13, 14, and 15. If we have 5 copies of card 10, we win 5 additional
copies of cards 11, 12, 13, 14, and 15.

The goal is to find the total number of cards.

### Approach
First, we initialize a tracker to keep track of how many copies of each
card we've won so far.

Then, for each game:
- split each card into winning numbers and our numbers
- map each winning number into a HashMap entry for lookup
- calculate number of winning numbers for each game
- get total number of cards by checking the "extra cards" tracker
- increment "extra cards" tracker based on number of matched numbers and total cards
- increment total

### Answer
>! 13768818
