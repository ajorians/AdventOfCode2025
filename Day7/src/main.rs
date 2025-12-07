use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use crate::ObjectType::Beam;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum ObjectType {
    Nothing,
    Start,
    Splitter,
    Beam,
    Unknown
}

struct Manifold
{
    data: Vec<Vec<ObjectType>>,
}

impl Manifold
{
    fn get_width(&self) -> i32
    {
        return self.data[0].len() as i32;
    }
    fn get_height(&self) -> i32
    {
        return self.data.len() as i32;
    }
    fn get_at(&self, x: i32, y: i32) -> ObjectType
    {
        let object_type: ObjectType = self.data[y as usize][x as usize];
        return object_type;
    }
    fn set_at(&mut self, x: i32, y: i32, newValue: ObjectType) {
        self.data[y as usize][x as usize] = newValue;
    }
}

fn build_manifold(lines: Vec<String>) -> Manifold {
    let mut manifold_vec: Vec<Vec<ObjectType>> = Vec::new();
    for line in lines
    {
        let mut row: Vec<ObjectType> = Vec::new();
        for ch in line.chars()
        {
            let spot: ObjectType =
                if ch == '.'
                {
                    ObjectType::Nothing
                } else if ch == 'S'
                {
                    ObjectType::Start
                } else if ch == '^'
                {
                    ObjectType::Splitter
                }
                else {
                    ObjectType::Unknown
                };

            row.push(spot);
        }
        manifold_vec.push(row);
    }

    let manifold: Manifold = Manifold
    {
        data: manifold_vec
    };

    return manifold;
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let lines = lines_from_file(file_path);

    let mut manifold = build_manifold(lines);

    run_beam( &mut manifold );
}

fn run_beam(manifold: &mut Manifold) {
    let mut numSplits = 0;

    for y in 0..manifold.get_height()
    {
        for x in 0..manifold.get_width()
        {
            if manifold.get_at(x,y) == ObjectType::Start && (y+1) < manifold.get_height()
            {
                manifold.set_at( x, y+1, Beam );
            }

            if manifold.get_at(x,y) == ObjectType::Beam && (y+1) < manifold.get_height()
            {
                if( manifold.get_at(x, y+1 ) == ObjectType::Nothing )
                {
                    manifold.set_at(x, y+1, Beam );
                }
                else if( manifold.get_at(x, y+1 ) == ObjectType::Splitter )
                {
                    numSplits += 1;
                    if( x > 1 )
                    {
                        manifold.set_at(x - 1, y + 1, Beam);
                    }
                    if( x+1 < manifold.get_width() )
                    {
                        manifold.set_at(x + 1, y + 1, Beam);
                    }
                }
            }
        }
    }

    println!("Number of splits: {}", numSplits);
}
