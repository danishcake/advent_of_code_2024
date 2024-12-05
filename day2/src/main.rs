use nom::character::complete::multispace0;
use nom::Err::Error;
use nom::{
    character::complete::{digit1, space1},
    multi::many1,
    IResult,
};

/// Parses a single number and converts to an i32
fn parse_i32(input: &str) -> IResult<&str, i32> {
    let (input, value) = digit1(input)?;
    let value: i32 = value
        .parse()
        .map_err(|_| Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit)))?;
    Ok((input, value))
}

/// Parses a row and returns the pair of entries
fn parse_row(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, row) = nom::multi::separated_list1(space1, parse_i32)(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, row))
}

/// Parses the input
fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    many1(parse_row)(input)
}

fn is_safe(row: &Vec<i32>) -> bool {
    let deltas: Vec<i32> = row.windows(2).map(|window| window[0] - window[1]).collect();

    // Do all the deltas have the same sign?
    // Determine this by counting the positive deltas. This must be either 0 or deltas.length
    let positive_delta_count = deltas
        .iter()
        .filter_map(|f| if f.signum() == 1 { Some(()) } else { None })
        .count();

    // Do all the deltas have a value between 1 and 3?
    let large_delta_count = deltas
        .iter()
        .filter_map(|f| match f {
            1..=3 => None,
            -3..=-1 => None,
            _ => Some(()),
        })
        .count();

    return (positive_delta_count == 0 || positive_delta_count == deltas.len())
        && large_delta_count == 0;
}

/// Solves part 1
/// A row is considered safe if all elements are
/// 1. Either increasing or decreasing
/// 2. Between 1-3 inclusive different
fn part_1(rows: &Vec<Vec<i32>>) {
    let safe_rows = rows.iter().filter(|&row| is_safe(row)).count();
    println!("Part 1: {}", safe_rows);
}

/// Solves part 2
/// Here each row is tested, and if found unsafe, we try removing one element in turn
fn part_2(rows: &Vec<Vec<i32>>) {
    let safe_rows = rows
        .iter()
        .filter(|&row| {
            if is_safe(row) {
                // Safe without further modification
                return true;
            }

            // Otherwise, let's try dropping each element in turn and checking again
            // Pretty ghastly solution
            for dropped_index in 0..row.len() {
                let mut row_without_element: Vec<i32> = row.clone();
                row_without_element.remove(dropped_index);

                if is_safe(&row_without_element) {
                    // Safe without further modification
                    return true;
                }
            }

            false
        })
        .count();
    println!("Part 2: {}", safe_rows);
}

fn main() {
    // Read the input
    let input = std::fs::read_to_string("input.txt").expect("Unable to load input.txt");

    // Parse the rows
    let (_, rows) = parse(&input).expect("Unable to parse input");

    part_1(&rows);
    part_2(&rows);
}
