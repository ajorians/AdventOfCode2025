use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum PieceType {
    Roll,
    Nothing,
    Unknown
}

struct Room
{
    room_data: Vec<Vec<PieceType>>,
}

impl Room
{
    fn get_width(&self) -> i32
    {
        return self.room_data[0].len() as i32;
    }
    fn get_height(&self) -> i32
    {
        return self.room_data.len() as i32;
    }
    fn get_at(&self, x: i32, y: i32) -> PieceType
    {
        let piece_type: PieceType = self.room_data[y as usize][x as usize];
        return piece_type;
    }

    fn getNumSurroundingRolls(&self, x: i32, y: i32) -> i32 {
        let mut numSurroundingRolls = 0;

        if(x > 0 && y > 0 && self.get_at( x-1, y-1 ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(y > 0 && self.get_at( x, y-1 ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(x+1 < self.get_width() && y > 0 && self.get_at( x+1, y-1 ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(x > 0 && self.get_at( x-1, y ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(x+1 < self.get_width() && self.get_at( x+1, y ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(x > 0 && y+1 < self.get_height() && self.get_at( x-1, y+1 ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(y+1 < self.get_height() && self.get_at( x, y+1 ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }
        if(x+1 < self.get_width() && y+1 < self.get_height() && self.get_at( x+1, y+1 ) == PieceType::Roll )
        {
            numSurroundingRolls += 1;
        }

        return numSurroundingRolls;
    }
}

fn build_room(lines: Vec<String>) -> Room {
    let mut room_vec: Vec<Vec<PieceType>> = Vec::new();
    for line in lines
    {
        let mut row: Vec<PieceType> = Vec::new();
        for ch in line.chars()
        {
            let spot: PieceType =
                if ch == '@'
                {
                    PieceType::Roll
                } else if ch == '.'
                {
                    PieceType::Nothing
                } else {
                    PieceType::Unknown
                };

            row.push(spot);
        }
        room_vec.push(row);
    }

    let room: Room = Room
    {
        room_data: room_vec
    };

    return room;
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let lines = lines_from_file(file_path);

    let room = build_room(lines);

    let numSpots = findNumSpots( room, 4 );

    println!("Num spots with fewer than 4: {}", numSpots);
}

fn findNumSpots(room: Room, limitSpots: i32) -> i32 {
    let width = room.get_width();
    let height = room.get_height();

    let mut numSpots = 0;
    for y in 0..height
    //for y in 6..7
    {

        for x in 0..width
        //for x in 7..8
        {
        let spot: PieceType = room.get_at(x as i32, y as i32);
            if spot != PieceType::Roll
            {
                continue;
            }

            let surroundingRolls = room.getNumSurroundingRolls( x, y );

            println!("Spot ({}, {}) had {} surrounding", x, y, surroundingRolls);

            if( surroundingRolls < limitSpots )
            {

                numSpots += 1;
            }
        }
    }

    return numSpots;
}