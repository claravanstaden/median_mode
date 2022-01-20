use std::collections::HashMap;
use std::io;

fn main() {
    println!("Enter numbers separated by a space");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input_itr = input.split_whitespace();

    let mut numbers: Vec<u32> = Vec::new();

    for s in input_itr {
        let num = s.trim().parse().expect("Invalid number");

        numbers.push(num);
    }

    println!("{:?}", numbers);

    numbers.sort();

    println!("{:?}", numbers);

    let numbers_count = numbers.iter().len();

    println!("There are {} numbers.", numbers_count);

    let mut median = 0.0;

    if numbers_count % 2 == 0 {
        let bottom_middle = match numbers.get(numbers_count / 2) {
            None => {
                println!("No number found at index {}.", numbers_count / 2);
                return;
            }
            Some(bottom_middle) => *bottom_middle as f64
        };

        let top_middle = match numbers.get(numbers_count / 2 - 1) {
            None => {
                println!("No number found at index {}.", numbers_count / 2 - 1);
                return;
            }
            Some(top_middle) => *top_middle as f64
        };

        median = (bottom_middle + top_middle) / 2f64;
    }

    println!("The median is {}", median);

    let mut map = HashMap::new();

    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1
    }

    println!("Hashmap list {:#?}", map);

    let mut highest = 0;
    for (num, count) in &map {
        if *count > highest {
            highest = *count;
        }
    }

    let mut modes = Vec::new();
    for (num, count) in &map {
        if *count == highest {
            modes.push(num);
        }
    }

    println!("Mode is {:#?}", modes);
}
