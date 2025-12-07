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

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut numbers: Vec<Vec<i64>> = Vec::new();

    let columns : Vec<&str> = contents.first().unwrap().split_whitespace().collect();
    for i in 0..columns.len()
    {
        let column : Vec<i64> = Vec::new();
        numbers.push( column );
    }

    let mut sum: i64 = 0;
    for s in contents {
        let values: Vec<&str> = s.split_whitespace().collect();

        let first = values.first().unwrap().to_string();
        if( first.parse::<i64>().is_ok() ) {

            for i in 0..values.len()
            {
                let v = values[i].parse::<i64>().unwrap();
                numbers[i].push(v);
            }
        }
        else {
            for i in 0..values.len()
            {
                let op : String = values.get(i).unwrap().to_string();
                let result = DoOperation(numbers[i].clone(), op);
                sum += result;
            }
        }
    }
    println!("Total sum: {}", sum);
}

fn DoOperation(values: Vec<i64>, operation: String) -> i64 {
    let mut result : i64 = values.first().unwrap().clone();

    for i in 1..values.len()
    {
        let v = values.get(i).unwrap().clone();
        if( operation == "*" )
        {
            result *= v;
        }
        else if( operation == "+" )
        {
            result += v;
        }
    }

    return result;
}
