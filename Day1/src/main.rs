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

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = lines_from_file(file_path);

    let mut timesHitZero = 0;
    let mut currentLocation = 50;
    for line in contents {
        let direction = &line[0..1];
        let amountStr = &line[1..];

        let amount = amountStr.parse::<i32>().unwrap();

        if( direction == "R" )
        {
            currentLocation += amount;
        }
        else {
            currentLocation -= amount;
        }

        while( currentLocation < 0 )
        {
            currentLocation += 100
        }
        while( currentLocation > 99 )
        {
            currentLocation -= 100;
        }

        if( currentLocation == 0)
        {
            timesHitZero += 1;
        }

        println!("Current location: {:?}", currentLocation);
    }

    println!("Total times hit 0: {:?}", timesHitZero);
}
