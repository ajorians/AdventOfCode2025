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

#[derive(PartialEq)]
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

    // let mut numFreshIngredients = 0;
    // for i in ingredients
    // {
    //     let isFresh : bool = is_fresh( &ranges, i );
    //
    //     println!("Ingredient {} is {}", i, isFresh );
    //
    //     if( isFresh )
    //     {
    //         numFreshIngredients += 1;
    //     }
    // }

    let numFreshIngredients = get_num_fresh( &mut ranges );
    println!("There are {} fresh ingredients", numFreshIngredients );
}

fn get_num_fresh(freshRanges: &mut Vec<Range>) -> i64 {

    freshRanges.sort_by(|a, b| a.start.cmp(&b.start));

    for r in freshRanges.iter().clone() {
        println!("Range {}-{}", r.start, r.end);
    }
    println!("FIXING RANGES");

    //Reduce overlap
    reduceOverlap( freshRanges );

    let mut numFreshIngredients = 0;
    for r in freshRanges {
        let num = r.end - r.start + 1;
        println!("Range {}-{} : ({})", r.start, r.end, num );
        numFreshIngredients += num;
    }

    return numFreshIngredients;
}

fn reduceOverlap(ranges: &mut Vec<Range>) {
    'outer: loop {
        for i in 0..ranges.len() {
            for j in 0..ranges.len()
            {
                if (i == j) {
                    continue;
                }

                let a = &ranges[i];
                let b = &ranges[j];
                if (a.start >= b.start && a.end <= b.end)
                {
                    ranges.remove(i);
                    continue 'outer;
                }

                if (b.start >= a.start && b.end <= a.end)
                {
                    ranges.remove(j);
                    continue 'outer;
                }

                if( a.start < b.start && a.end >= b.start )
                {
                    ranges[i].end = b.start-1;
                    continue;
                }

                if( a.start <= b.end && a.end > b.end )
                {
                    ranges[i].start = b.end+1;
                    continue;
                }
            }
        }

        break;
    }
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