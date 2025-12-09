use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use std::cmp::PartialEq;
use regex::Regex;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(PartialEq)]
#[derive(Clone)]
struct Coordinate
{
    x: i64,
    y: i64,
    z: i64
}

fn main() {
    let file_path = "basic_input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let coords: Vec<Coordinate> = build_coordinates(contents);

    let junctions = find_junctions( &coords );

    let mut result = 1;
    for junction in &junctions {
        println!("Junction has {} items", junctions.len());
        result *= junctions.len();
    }

    println!("Answer is: {}", result);
}

fn find_junctions(coordinates: &Vec<Coordinate>) -> Vec<Vec<Coordinate>>{
    let mut junctions : Vec<Vec<Coordinate>> = Vec::new();

    for i in 0..10 {
        let (a, b) = find_closest( &junctions, &coordinates );

        if( a == None &&b == None ){
            break;
        }

        AddToJunction( &mut junctions, a.unwrap(), b.unwrap() );
    }

    return junctions;
}

fn find_closest(junctions: &Vec<Vec<Coordinate>>, coordinates: &Vec<Coordinate>) -> (Option<Coordinate>, Option<Coordinate>) {
    let mut closest = f64::MAX;
    let mut closest_aIndex = 0;
    let mut closest_bIndex = 0;
    for i in 0..coordinates.len()
    {
        let first = coordinates.get(i).unwrap();

        if( isInJunction( junctions, first))
        {
            continue;
        }

        for j in 0..coordinates.len() {
            if i == j
            {
                continue;
            }

            let second = coordinates.get(j).unwrap();

            let distance = (((second.x - first.x).abs().pow(2) + (second.y - first.y).abs().pow(2) + (second.z - first.z).abs().pow(2)) as f64).sqrt();
            if distance < closest {
                closest = distance;
                closest_aIndex = i;
                closest_bIndex = j;
            }

        }
    }

    // if ( closest_aIndex == 0 && closest_bIndex == 0 )
    // {
    //    return (None, None);
    // }

    let a = coordinates.get(closest_aIndex).unwrap();
    let b = coordinates.get(closest_bIndex).unwrap();
    return (Some(a.clone() ), Some( b.clone() ));
}

fn isInJunction(junctions: &Vec<Vec<Coordinate>>, testItem: &Coordinate) -> bool {
    for junction in junctions.iter()
    {
        for item in junction.iter()
        {
            if *item == *testItem
            {
                return true;
            }
        }
    }
    return false;
}

fn AddToJunction(junctions: &mut Vec<Vec<Coordinate>>, a: Coordinate, b: Coordinate) {
    let mut matching_junction_index: Option<usize> = None;
    let mut has_a = false;
    let mut has_b = false;
    for junctionIndex in 0..junctions.len()
    {
        let testJunction = junctions[junctionIndex].clone();
        for item in testJunction
        {
            if item == a
            {
                matching_junction_index = Some(junctionIndex);
                has_a = true;
            }
            if item == b
            {
                matching_junction_index = Some(junctionIndex);
                has_b = true;
            }
        }
    }

    if (matching_junction_index.is_some())
    {
        if( !has_a )
        {
            let junctionToAddTo = &mut junctions[matching_junction_index.unwrap()];
            junctionToAddTo.push(a);
        }
        if( !has_b )
        {
            let junctionToAddTo = &mut junctions[matching_junction_index.unwrap()];
            junctionToAddTo.push(b);
        }
    } else {
        let mut newJunction: Vec<Coordinate> = Vec::new();
        newJunction.push(a);
        newJunction.push(b);
        junctions.push(newJunction);
    }
}

fn build_coordinates(lines: Vec<String>) -> Vec<Coordinate> {
    let mut coord_vec: Vec<Coordinate> = Vec::new();

    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

    for line in lines
    {
        let captures = re.captures(&line).unwrap();

        let cord: Coordinate = Coordinate
        {
            x : captures.get(1).unwrap().as_str().parse().unwrap(),
            y : captures.get(2).unwrap().as_str().parse().unwrap(),
            z : captures.get(3).unwrap().as_str().parse().unwrap()
        };

        coord_vec.push(cord);
    }

    return coord_vec;
}

