// 记得放 myfile.csv
use nom::{
    bytes::complete::is_not,
    character::complete::{char, line_ending},
    combinator::opt,
    multi::separated_list0,
    sequence::terminated,
    IResult,
};
use std::{fs, io::Error};

fn read_file(file_path: &str) -> Result<String, Error> {
    fs::read_to_string(file_path)
}

// parse_csv => parser
fn parse_csv(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    println!("input csv file:");
    println!("{}", input);
    // terminated => combinator
    // line_ending => parser
    // opt => combinator
    separated_list0(terminated(line_ending, opt(line_ending)), parse_line)(input)
}

// parse_line => parser
fn parse_line(input: &str) -> IResult<&str, Vec<&str>> {
    // separated_list0 => a combinator
    // accepts 2 parser
    separated_list0(char(','), is_not(",\r\n"))(input)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = read_file("/path/to/myfile.csv")?;
    match parse_csv(&file_content) {
        Ok((_, rows)) => {
            for row in rows {
                println!("{:?}", row);
            }
        }
        Err(e) => println!("Failed to parse CSV: {:?}", e),
    }
    Ok(())
}

