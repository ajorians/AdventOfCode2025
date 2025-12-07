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

    let mut numbers: Vec<Vec<String>> = Vec::new();

    let mut indexes : Vec<usize> = Vec::new();
    for (byte_index, character) in contents.last().unwrap().char_indices() {
        if( character == '+' || character == '*' )
        {
            indexes.push(byte_index);
        }
    }

    for i in 0..indexes.len()
    {
        let column : Vec<String> = Vec::new();
        numbers.push( column );
    }

    let mut sum: i64 = 0;
    for s in contents {
        for i in 0..indexes.len()
        {
            let mut value : String = String::new();
            if( i + 1 < indexes.len() )
            {
                value = s[indexes[i]..indexes[i+1]].to_string();
            }
            else {
                value = s[indexes[i]..].to_string();
            }

            numbers[i].push( value );
        }
    }

    let mut sum : i64 = 0;
    for i in 0..numbers.len()
    {
        let cephopodNumbers = ConvertToCephopod( numbers[i].clone() );
        let val = DoOperation( cephopodNumbers.clone() );
        sum += val;
    }

    println!("Total sum: {}", sum);
}

fn ConvertToCephopod(values: Vec<String>) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();

    let mut max_len= 0;
    for str in values.clone()
    {
        let length = str.len();
        if( length > max_len )
        {
            max_len = length;
        }
    }

    let usefulLength = max_len-1;

    for digit in (0..=usefulLength).rev()
    {
        let mut newNumber : String = String::new();
        for i in 0..values.len()-1
        {
            let ch = values[i].chars().nth( digit ).unwrap();

            if ch == ' '
            {
                continue;
            }

            newNumber.push( ch );
        }

        if( newNumber.is_empty() )
        {
            continue;
        }

        result.push( newNumber );
    }

    result.push( values.last().unwrap().clone() );

    return result;
}

fn DoOperation(values: Vec<String>) -> i64 {
    let operation = values.last().unwrap().clone().trim().to_string();

    let mut result : i64 = values.first().unwrap().clone().trim().to_string().parse::<i64>().unwrap();

    for i in 1..values.len()-1
    {
        let v = values.get(i).unwrap().clone().trim().to_string();
        if( operation == "*" )
        {
            result *= v.parse::<i64>().unwrap();
        }
        else if( operation == "+" )
        {
            result += v.parse::<i64>().unwrap();
        }
    }

    return result;
}

