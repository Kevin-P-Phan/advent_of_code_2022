use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    match read_lines("src/input.txt") {
        Ok(lines) => {
            let mut points: i32 = 0;
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(game) = line {
                    println!("Current match is {}", game);
                    let result: Vec<&str> = game.split(' ').collect();
                    // A is rock, B is paper, C is scissors
                    // X is rock, Y is paper, Z is scissors
                    if ("A" == result[0] && result[1] == "Y")
                        || ("B" == result[0] && result[1] == "Z")
                        || ("C" == result[0] && result[1] == "X")
                    {
                        points += 6;
                    } else if ("A" == result[0] && result[1] == "X")
                        || ("B" == result[0] && result[1] == "Y")
                        || ("C" == result[0] && result[1] == "Z")
                    {
                        points += 3;
                    }
                    if result[1] == "X" {
                        points += 1;
                    } else if result[1] == "Y" {
                        points += 2;
                    } else {
                        points += 3;
                    }
                }
            }
            println!("The sum of the points is {}.", points)
        }
        Err(_) => println!("You did not supply the correct input file."),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
