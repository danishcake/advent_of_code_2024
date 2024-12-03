use nom::character::complete::{multispace0, space1};
use nom::multi::many1;
use nom::Err::Error;
use nom::{character::complete::digit1, IResult};

/// A single row of data
#[derive(Debug, PartialEq)]
struct Row {
    pub lhs: i32,
    pub rhs: i32,
}

/// Parses a single number and converts to an i32
fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, value) = digit1(input)?;
    let value: i32 = value
        .parse()
        .map_err(|_| Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))?;
    Ok((input, value))
}

/// Parses a row and returns the pair of entries
fn parse_row(input: &str) -> IResult<&str, Row> {
    let (input, lhs) = parse_i32(input)?;
    let (input, _) = space1(input)?;
    let (input, rhs) = parse_i32(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Row { lhs, rhs }))
}

/// Parses the input
fn parse(input: &str) -> IResult<&str, Vec<Row>> {
    many1(parse_row)(input)
}

/// Solves part 1
fn part_1(mut lhs: Vec<i32>, mut rhs: Vec<i32>) {
    // Sort the vectors
    lhs.sort();
    rhs.sort();

    // Zip and calculate the sum of the differences
    let sum: u32 = lhs
        .iter()
        .zip(rhs)
        .map(|(lhs, rhs)| lhs.abs_diff(rhs))
        .sum();
    println!("Part 1: {}", sum);
}

/// Solves part 2
fn part_2(lhs: Vec<i32>, rhs: Vec<i32>) {
    // For each element, count the number of times it occurs in the second list
    // If there were larger inputs we might want to consider efficiency here, but n*m is only 1,000,0000
    let count_product: usize = lhs
        .iter()
        .map(|l| rhs.iter().filter(|&r| r == l).count() * (*l as usize))
        .sum();

    println!("Part 2: {}", count_product);
}

fn main() {
    // Read the input
    let input = std::fs::read_to_string("input.txt").expect("Unable to load input.txt");

    // Parse the rows
    let (_, rows) = parse(&input).expect("Unable to parse input");

    // Split into two vectors
    let lhs: Vec<i32> = rows.iter().map(|f| f.lhs).collect();
    let rhs: Vec<i32> = rows.iter().map(|f| f.rhs).collect();

    part_1(lhs.clone(), rhs.clone());
    part_2(lhs.clone(), rhs.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    /// GIVEN Example input
    /// WHEN parsed
    /// THEN the expected rows are generated
    #[test]
    fn test_input_parsing() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        let expected = vec![
            Row { lhs: 3, rhs: 4 },
            Row { lhs: 4, rhs: 3 },
            Row { lhs: 2, rhs: 5 },
            Row { lhs: 1, rhs: 3 },
            Row { lhs: 3, rhs: 9 },
            Row { lhs: 3, rhs: 3 },
        ];
        let (_, actual) = parse(input).unwrap();
        assert_eq!(actual, expected);
    }
}
