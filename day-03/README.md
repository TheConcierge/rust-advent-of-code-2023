# Day 3
## Link to puzzle
https://adventofcode.com/2023/day/3
## Part 1
### Problem Definition
We are given an input consisting of character arranged in a rectangle. There are three categories of symbols:
- the '.' character
- digits
- symbols (anything that isn't a digit or a '.')

Concurrent digits constitute a number. Any number that is adjacent (even diagonally) to a symbol is a "part number" and
should be used to calculate the answer.

The answer is the sum of all "part numbers".

### Approach
In order to solve this problem, I decided to parse the input twice. The first time, I store the location of all symbols in
a trie. The second time, I find all numbers, calculate all adjacent spaces, and see if the coordinates exist in the trie. 

If the coordinates exist in the trie, that means there is an adjacent symbol and the number should be included in our total.

### Notes
- using a trie here was completely unnecessary, the same can be accomplished with a HashMap (this was adjusted in part 2)

### Solution
>! 509115

## Part 2
### Problem Defintion
This time, the type of symbol actually matters! "*" symbols are gears, but *only* if they are adjacent to two numbers. Gears
have a "gear ratio", which is the product of multiplying the part numbers.

The answer is the sum of all "gear ratios".

### Approach
Part 2 used a similar "double parse" method. The distinction this time is that the first pass parses out all the numbers. During
the number parsing, I create a lookup table, mapping the coordinate of each character to the actual value.

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

The number 467 gets stored in the lookup table as:
"0, 0" -> 467,
"0, 1" -> 467,
"0, 2" -> 467

After parsing all of the numbers, I parse the input again, looking for "*". When we hit a "*", we calculate all adjacent 
coordinates and see if they exist in the hashmap. If two parts exist in adjacent squares, we can multiply them and add
them to our total

### Notes
- this approach assumes that each part number on a given gear is unique 

### Answer
>! 75220503
