use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;

#[derive(PartialEq)]
#[derive(Clone)]
struct Coordinate
{
    x: i64,
    y: i64
}

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

    let coordinates = build_coordinates( contents );

    find_biggest_area( coordinates );
}

fn find_biggest_area(coordinates: Vec<Coordinate>) {
    let mut largestArea = 0;
    for topLeftIndex in 0..coordinates.len()
    {
        for bottomRightIndex in 0..coordinates.len()
        {
            if( topLeftIndex == bottomRightIndex )
            {
                continue;
            }

            let topLeft = coordinates[topLeftIndex].clone();
            let bottomRight = coordinates[bottomRightIndex].clone();

            if( topLeft.x > bottomRight.x )
            {
                continue;
            }

            if( topLeft.y > bottomRight.y )
            {
                continue;
            }

            let w = bottomRight.x - topLeft.x + 1;
            let h = bottomRight.y - topLeft.y + 1;
            let area = w * h;

            if( area > largestArea )
            {
                largestArea = area;
            }
        }
    }

    println!( "Largest area: {}", largestArea );
}

fn build_coordinates(lines: Vec<String>) -> Vec<Coordinate> {
    let mut coord_vec: Vec<Coordinate> = Vec::new();

    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    for line in lines
    {
        let captures = re.captures(&line).unwrap();

        let cord: Coordinate = Coordinate
        {
            x : captures.get(1).unwrap().as_str().parse().unwrap(),
            y : captures.get(2).unwrap().as_str().parse().unwrap()
        };

        coord_vec.push(cord);
    }

    return coord_vec;

}