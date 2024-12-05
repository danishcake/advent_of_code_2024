use nom::character::complete::{anychar, line_ending};
use nom::multi::many_till;
use nom::{multi::many1, IResult};

/// Parses one row of input.
/// This just takes characters until the newline
fn parse_row(input: &str) -> IResult<&str, Vec<char>> {
    let (input, (chars, _)) = many_till(anychar, line_ending)(input)?;

    Ok((input, chars))
}

/// Parses the input
fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(parse_row)(input)
}

/// Solves part 1
fn part_1(rows: &Vec<Vec<char>>) -> usize {
    let fneedle = vec!['X', 'M', 'A', 'S'];
    let rneedle = fneedle.iter().rev().cloned().collect::<Vec<_>>();
    let needle_len = fneedle.len();
    let mut sample = fneedle.clone();
    let mut count = 0usize;

    for y in 0..rows.len() {
        let row = &rows[y];
        for x in 0..row.len() {
            // Scan down
            if y <= rows.len() - needle_len {
                for i in 0..needle_len {
                    sample[i] = rows[y + i][x];
                }
                if sample == fneedle {
                    count += 1;
                }
                if sample == rneedle {
                    count += 1;
                }
            }

            // Scan right-down
            if y <= rows.len() - needle_len && x <= row.len() - needle_len {
                for i in 0..needle_len {
                    sample[i] = rows[y + i][x + i];
                }
                if sample == fneedle {
                    count += 1;
                }
                if sample == rneedle {
                    count += 1;
                }
            }

            // Scan right
            if x <= row.len() - needle_len {
                for i in 0..needle_len {
                    sample[i] = rows[y][x + i];
                }
                if sample == fneedle {
                    count += 1;
                }
                if sample == rneedle {
                    count += 1;
                }
            }

            // Scan right-up
            if y >= needle_len - 1 && x <= row.len() - needle_len {
                for i in 0..needle_len {
                    sample[i] = rows[y - i][x + i];
                }
                if sample == fneedle {
                    count += 1;
                }
                if sample == rneedle {
                    count += 1;
                }
            }
        }
    }

    count
}

const NEEDLE_SIZE: usize = 3usize;
type Patch = [[char; NEEDLE_SIZE]; NEEDLE_SIZE];
/// Solves part 2
/// Here I need to find 3x3 patterns that contain MAS. There are four permutations of this
///
/// M S
///  A
/// M S
///
/// M M
///  A
/// S S
///
/// S M
///  A
/// S M
///
/// S S
///  A
/// M M
/// I think this whole problem was meant to be solved with a masked comparison, but I'm committed now!
fn part_2(rows: &Vec<Vec<char>>) -> usize {
    let needles: Vec<Patch> = vec![
        [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
        [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
        [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
        [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
    ];

    let mut count = 0usize;

    for y in 0..=rows.len() - NEEDLE_SIZE {
        let row = &rows[y];
        for x in 0..=row.len() - NEEDLE_SIZE {
            // Create a Patch from this
            let sample: Patch = [
                [rows[y + 0][x], '.', rows[y + 0][x + 2]],
                ['.', rows[y + 1][x + 1], '.'],
                [rows[y + 2][x], '.', rows[y + 2][x + 2]],
            ];
            for needle in needles.iter() {
                if needle == &sample {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    // Read the input
    let input = std::fs::read_to_string("input.txt").expect("Unable to load input.txt");

    // Parse the rows
    let (_, rows) = parse(&input).expect("Unable to parse input");

    println!("Part 1: {}", part_1(&rows));
    println!("Part 2: {}", part_2(&rows));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GIVEN Example input
    /// WHEN parsed and result calculated
    /// THEN the expected result is found
    #[test]
    fn test_part2_matching() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let (_, rows) = parse(input).unwrap();
        let expected = 9;
        let actual = part_2(&rows);

        assert_eq!(expected, actual);
    }
}
