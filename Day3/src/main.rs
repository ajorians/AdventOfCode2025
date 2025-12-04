use std::cmp::max;
use std::{
    fs::File,
    io::{BufReader, prelude::*},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut sum = 0;
    for line in contents {
        let largestNumber = GetLargestNDigitNumber(&line, 12);
        println!("Largest Number in {} is {}", line, largestNumber);
        sum += largestNumber;
    }

    println!("Sum: {}", sum);
}

fn GetLargestTwoDigit(text: &String) -> i32 {
    let mut largetNumber = 0;

    let numChars = text.len();
    for i in 0..numChars {
        for j in i + 1..numChars {
            let a = text.chars().nth(i).unwrap();
            let b = text.chars().nth(j).unwrap();

            let first = a.to_digit(10).unwrap() as i32 * 10;
            let second = b.to_digit(10).unwrap() as i32;

            largetNumber = max(largetNumber, first + second);
        }
    }

    return largetNumber;
}

fn GetLargestNDigitNumber(text: &String, numDigitsResult: usize) -> i64 {
    let numChars = text.len();
    let mut digitVec: Vec<i32> = Vec::new();

    digitVec.resize(numDigitsResult, 0);

    let mut startIndex = 0;
    for digitPlace in 0..numDigitsResult {
        let lastIndex = numChars - (numDigitsResult - digitPlace);
        for i in startIndex..=lastIndex {
            let ch = text.chars().nth(i).unwrap();
            let n = ch.to_digit(10).unwrap() as i32;

            if (n > digitVec[digitPlace]) {
                digitVec[digitPlace] = n;
                startIndex = i + 1;
            }
        }
    }

    let joined: String = digitVec.iter().map( |&id| id.to_string()).collect();

    let largetNumber: i64 = joined.parse().unwrap();

    return largetNumber;
}
