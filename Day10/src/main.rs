use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
use regex::Regex;

#[derive(PartialEq)]
#[derive(Clone)]
struct Button
{
    Changes : Vec<i32>
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

    let mut totalMoves = 0;
    for line in contents {
        let minMoves = RunLine( line.clone() );

        println!("For line: {}; total Moves: {}", line, minMoves);

        totalMoves += minMoves;
    }

    println!( "Total Moves: {}", totalMoves  );
}

fn RunLine(line: String) -> i32 {
    let re = Regex::new(r"\[([.#]+)\]([^{]*)(\{[\d,]+\})").unwrap();

    let captures = re.captures(&line).unwrap();
    let pattern = captures.get(1).unwrap().as_str();
    let buttonsText = captures.get(2).unwrap().as_str();

    //println!( "pattern: {}", pattern);
    //println!( "Buttons: {}", buttonsText);

    let buttons : Vec<Button> = BuildButtons( buttonsText );

    let length = pattern.len();
    let initial = ".".repeat(length);

    let totalMoves = ComputeSteps( initial, pattern, buttons );

    return totalMoves;
}

fn ComputeSteps(start: String, end: &str, buttons: Vec<Button>) -> i32 {

    let mut changedStates : Vec<String> = Vec::new();
    changedStates.push( start );

    let mut totalMoves : i32 = 0;
    loop
    {
        totalMoves += 1;

        let mut newStates : Vec<String> = Vec::new();
        for startingText in changedStates.iter()
        {
            for i in 0..buttons.len()
            {
                let changedState = ApplyChange(startingText.clone(), buttons[i].clone());

                if( changedState == end )
                {
                    return totalMoves;
                }

                //println!("Changed State: {}", changedState);
                newStates.push(changedState);
            }
        }

        changedStates.clear();
        changedStates = newStates;
    }

    return 0;
}

fn ApplyChange(start: String, button: Button) -> String {
    let mut result = start.clone();

    for change in button.Changes
    {
        let indexFlipped = change.clone() as usize;

        let ch = result.chars().nth(indexFlipped).unwrap();

        if( ch == '.' )
        {
            result.replace_range(indexFlipped..indexFlipped + 1, "#")
        }
        else {
            result.replace_range(indexFlipped..indexFlipped + 1, ".")
        }
    }

    return result;
}

fn BuildButtons(buttonsText: &str) -> Vec<Button> {
    let mut result : Vec<Button> = Vec::new();

    let re = Regex::new(r"\(([\d,]*)\)").unwrap();

    let captures = re.captures(&buttonsText).unwrap();

    for cap in re.captures_iter(buttonsText) {
        let inside = cap.get(1).unwrap().as_str();
        //println!("{}", inside);

        let values = inside.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let button : Button = Button
        {
            Changes : values
        };

        result.push( button );
    }

    return result;
}