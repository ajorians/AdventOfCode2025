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

    let mut ranges: Vec<Range> = Vec::new();
    let mut ingredients: Vec<i64> = Vec::new();

    let re = Regex::new(r"(\d+-\d+)").unwrap();

    let mut reachedItems = false;
    for s in contents {
        if( s.is_empty() )
        {
            reachedItems = true;
            continue;
        }

        if( !reachedItems )
        {
            for mat in re.find_iter(&s) {
                let rangeStr = mat.as_str().to_string();

                ranges.push(rangeParse(&rangeStr));
            }
        }
        else {
            ingredients.push( s.parse::<i64>().unwrap() );
        }
    }

    let mut numFreshIngredients = 0;
    for i in ingredients
    {
        let isFresh : bool = is_fresh( &ranges, i );

        println!("Ingredient {} is {}", i, isFresh );

        if( isFresh )
        {
            numFreshIngredients += 1;
        }
    }
    println!("There are {} fresh ingredients", numFreshIngredients );
}

fn is_fresh(freshRanges: &Vec<Range>, ingredient: i64) -> bool {
    for r in freshRanges
    {
        if( ingredient >= r.start && ingredient <= r.end )
        {
            return true;
        }
    }

    return false;
}