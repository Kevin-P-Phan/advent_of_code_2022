use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    match read_lines("src/input.txt") {
        Ok(lines) => {
            let mut part1_points: i32 = 0;
            let mut part2_points: i32 = 0;
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
                        part1_points += 6;
                    } else if ("A" == result[0] && result[1] == "X")
                        || ("B" == result[0] && result[1] == "Y")
                        || ("C" == result[0] && result[1] == "Z")
                    {
                        part1_points += 3;
                    }
                    points_from_shape(&result, &mut part1_points);
                    if result[1] == "Y" {
                        part2_points += 3;
                        points_from_opp_shape(&result, &mut part2_points);
                    } else if result[1] == "Z" {
                        part2_points += 6;
                        if result[0] == "A" {
                            part2_points += 2;
                        } else if result[0] == "B" {
                            part2_points += 3;
                        } else {
                            part2_points += 1;
                        }
                    } else {
                        if result[0] == "A" {
                            part2_points += 3;
                        } else if result[0] == "B" {
                            part2_points += 1;
                        } else {
                            part2_points += 2;
                        }
                    }
                }
            }
            println!("The sum of the points for part 1 is {}.", part1_points);
            println!("The sum of the points for part 2 is {}.", part2_points);
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

fn points_from_shape(result: &Vec<&str>, points: &mut i32) {
    if result[1] == "X" {
        *points += 1;
    } else if result[1] == "Y" {
        *points += 2;
    } else {
        *points += 3;
    }
}

fn points_from_opp_shape(result: &Vec<&str>, points: &mut i32) {
    if result[0] == "A" {
        *points += 1;
    } else if result[0] == "B" {
        *points += 2;
    } else {
        *points += 3;
    }
}
