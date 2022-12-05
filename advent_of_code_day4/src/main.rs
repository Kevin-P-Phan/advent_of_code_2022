use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    let mut sum2 = 0;
    // File hosts must exist in current path before this produces output
    match read_lines("src/input.txt") {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(assignment) = line {
                    let assignments: Vec<&str> = assignment.split(',').collect();
                    let assignment1: Vec<&str> = assignments[0].split('-').collect();
                    let assignment2: Vec<&str> = assignments[1].split('-').collect();
                    let start1: i32 = assignment1[0].parse().unwrap();
                    let end1: i32 = assignment1[1].parse().unwrap();
                    let start2: i32 = assignment2[0].parse().unwrap();
                    let end2: i32 = assignment2[1].parse().unwrap();
                    let range1 = start1..=end1;
                    let range2 = start2..=end2;
                    if (range1.contains(&range2.start()) && range1.contains(&range2.end()))
                        || (range2.contains(&range1.start()) && range2.contains(&range1.end()))
                    {
                        sum += 1;
                    }
                    if (range1.contains(&range2.start()) || range1.contains(&range2.end()))
                        || (range2.contains(&range1.start()) || range2.contains(&range1.end()))
                    {
                        sum2 += 1;
                    }
                }
            }
        }
        Err(_) => println!("You did not supply the correct input file."),
    }
    println!("The number of fully emcompass assignments is {}", sum);
    println!("The number of overlaping assignments is {}", sum2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
