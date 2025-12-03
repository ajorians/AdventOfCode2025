use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

struct Range
{
    start : i64,
    end : i64
}

fn rangeParse(map_line : &String) ->Range{
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();

    let captures = re.captures(&*map_line).unwrap();
    let start = &captures[1];
    let end = &captures[2];

    let result : Range = Range
    {
        start : start.parse().unwrap(),
        end : end.parse().unwrap()
    };

    return result;
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut ranges : Vec<Range> = Vec::new();
    let re = Regex::new(r"(\d+-\d+)").unwrap();
    for s in contents {
        for mat in re.find_iter(&s) {
            let rangeStr = mat.as_str().to_string();

            ranges.push(rangeParse(&rangeStr));
        }
    }

    let mut sumInvalidIds = 0;
    for range in ranges{
        let invalidIds = findInvalidIds( &range );

        println!("For range ({}, {}) has {} invalid IDs", range.start , range.end , invalidIds.len());
        for invalidID in invalidIds{
            sumInvalidIds += invalidID;
            println!("{}", invalidID);
        }
    }

    println!("Sum of invalid Ids: {}", sumInvalidIds);
}

fn findInvalidIds(range: &Range) -> Vec<i64> {
    let mut invalidIds : Vec<i64> = Vec::new();

    for i in range.start ..= range.end {
        if( isInvalid(i) )
        {
            invalidIds.push( i as i64 );
        }
    }

    return invalidIds;
}

fn isInvalid(value: i64) -> bool {
    let str = value.to_string();

    let length = str.chars().count();
    let chars: Vec<char> = str.chars().collect();

    let first: String = chars.iter().take(length / 2).collect();
    let second: String = chars.iter().skip(length / 2).collect();

    return first == second;
}
