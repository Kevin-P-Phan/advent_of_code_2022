use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input.txt") {
        let mut sum = 0;
        let mut highest_calorie = 0;
        let mut total_calories = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calorie) = line {
                println!("Current calorie is {}", calorie);
                let blank = calorie.trim().is_empty();
                if blank && sum > highest_calorie
                {
                    highest_calorie = sum;
                    println!("The current highest calorie is {}", highest_calorie);
                    total_calories.push(highest_calorie);
                    sum = 0;
                }
                else if blank
                {
                    total_calories.push(sum);
                    sum = 0;
                }
                else
                {
                    sum = sum + calorie.parse::<i32>().unwrap();
                    println!("Current sum is {}", sum);
                }
            }
        }
        println!("The highest calorie is {}", highest_calorie);
        total_calories.sort();
        for x in &total_calories
        {
            println!("{}",x);
        }
        println!("{}", total_calories[total_calories.len()-3] + total_calories[total_calories.len()-2] + total_calories[total_calories.len()-1]);
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}