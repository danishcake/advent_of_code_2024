use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::char;
use nom::Err::Error;
use nom::{character::complete::digit1, multi::many1, IResult};

/// Represents an operation
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,

    // We'll get shed loads of these that we'll need to filter out
    // It would be more efficient to use many_till to throw away corrupted items, but
    // that's an optimisation
    Corrupted,
}

/// Parses a single number and converts to an i32
fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, value) = digit1(input)?;
    let value: i32 = value
        .parse()
        .map_err(|_| Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))?;
    Ok((input, value))
}

/// Parses a mul operation
fn parse_mul_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("mul(")(input)?;
    let (input, lhs) = parse_i32(input)?;
    let (input, _) = char(',')(input)?;
    let (input, rhs) = parse_i32(input)?;
    let (input, _) = char(')')(input)?;

    Ok((input, Operation::Mul(lhs, rhs)))
}

/// Reads a 'do' operation
fn parse_do_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("do")(input)?;
    Ok((input, Operation::Do))
}

/// Reads a 'don't' operation. Must be higher priority that do
fn parse_dont_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("don't")(input)?;
    Ok((input, Operation::Dont))
}

/// Reads a single byte unconditionally and returns a corrupted operation
fn parse_corrupted_operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = take(1usize)(input)?;
    Ok((input, Operation::Corrupted))
}

/// Parses a single operation
fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, operation) = alt((
        parse_mul_operation,
        parse_dont_operation,
        parse_do_operation,
        parse_corrupted_operation,
    ))(input)?;
    Ok((input, operation))
}

/// Parses the input
fn parse(input: &str) -> IResult<&str, Vec<Operation>> {
    many1(parse_operation)(input)
}

/// Solves part 1
fn part_1(operations: &Vec<Operation>) {
    let result: i32 = operations
        .iter()
        .filter(|&op| !matches!(op, Operation::Corrupted))
        .map(|op| match op {
            Operation::Mul(lhs, rhs) => lhs * rhs,
            Operation::Corrupted => unreachable!(),
            _ => {
                0 /* Not required for part 1 */
            }
        })
        .sum();

    println!("Part 1: {}", result);
}

/// Solves part 2
fn part_2(operations: &Vec<Operation>) {
    let mut enabled = 1i32;
    let mut sum = 0i32;
    for op in operations {
        match op {
            Operation::Mul(lhs, rhs) => sum += lhs * rhs * enabled,
            Operation::Do => enabled = 1,
            Operation::Dont => enabled = 0,
            Operation::Corrupted => {}
        }
    }

    println!("Part 2: {}", sum);
}

fn main() {
    // Read the input
    let input = std::fs::read_to_string("input.txt").expect("Unable to load input.txt");

    // Parse the rows
    let (_, operations) = parse(&input).expect("Unable to parse input");

    part_1(&operations);
    part_2(&operations);
}
