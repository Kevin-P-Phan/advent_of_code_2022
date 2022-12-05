use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    let mut sum_part2 = 0;
    let mut line_counter = 0;
    let mut items_vec: VecDeque<String> = VecDeque::new();
    // File hosts must exist in current path before this produces output
    match read_lines("src/input.txt") {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(item) = line {
                    items_vec.push_front(item.clone());
                    let mid: usize = item.len() / 2;
                    let items: (&str, &str) = item.split_at(mid);
                    sum += check_items(items.0, items.1);
                    line_counter += 1;
                    if line_counter % 3 == 0 {
                        sum_part2 += check_items_vecs(&items_vec[0], &items_vec[1], &items_vec[2]);
                    }
                }
            }
        }
        Err(_) => println!("You did not supply the correct input file."),
    }
    println!("The sum of the priorties is {}", sum);
    println!("The sum of the group priorties is {}", sum_part2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn priorities_map_generator() -> HashMap<char, i32> {
    let mut upper_alphabet: VecDeque<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut lower_alphabet: VecDeque<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut priorities: HashMap<char, i32> = HashMap::new();
    for i in 27..=52 {
        let letter = upper_alphabet.pop_front().unwrap();
        priorities.insert(letter, i);
    }
    for i in 1..=26 {
        let letter = lower_alphabet.pop_front().unwrap();
        priorities.insert(letter, i);
    }
    return priorities;
}

fn check_items(item1: &str, item2: &str) -> i32 {
    let priorities: HashMap<char, i32> = priorities_map_generator();
    let mut priority: i32 = 0;
    let mut item1_chars: HashSet<char> = HashSet::new();
    let mut item2_chars: HashSet<char> = HashSet::new();
    for i in item1.chars() {
        item1_chars.insert(i);
    }
    for i in item2.chars() {
        item2_chars.insert(i);
    }
    for i in item1_chars.into_iter() {
        if item2_chars.contains(&i) {
            priority = *priorities.get(&i).unwrap();
        }
    }

    return priority;
}

fn check_items_vecs(item1: &str, item2: &str, item3: &str) -> i32 {
    let priorities: HashMap<char, i32> = priorities_map_generator();
    let mut priority: i32 = 0;
    let mut item1_chars: HashSet<char> = HashSet::new();
    let mut item2_chars: HashSet<char> = HashSet::new();
    let mut item3_chars: HashSet<char> = HashSet::new();
    for i in item1.chars() {
        item1_chars.insert(i);
    }
    for i in item2.chars() {
        item2_chars.insert(i);
    }
    for i in item3.chars() {
        item3_chars.insert(i);
    }
    for i in item1_chars.into_iter() {
        if item2_chars.contains(&i) && item3_chars.contains(&i) {
            priority = *priorities.get(&i).unwrap();
        }
    }

    return priority;
}
