use std::fs::File;
use std::io::prelude::*;

fn find_double(input: &[u32]) -> Option<(u32, u32)> {
    for num1 in input {
        for num2 in input {
            if num1 + num2 == 2020 {
                return Some((*num1, *num2));
            }
        }
    }
    None
}

fn find_triple(input: &[u32]) -> Option<(u32, u32, u32)> {
    for num1 in input {
        for num2 in input {
            for num3 in input {
                if num1 + num2 + num3 == 2020 {
                    return Some((*num1, *num2, *num3));
                }
            }
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/day1.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<u32> = contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string().parse::<u32>().unwrap())
        .collect();


    match find_double(&lines) {
        Some((num1, num2)) => {
            println!("{} * {} = {}", num1, num2, num1 * num2)
        }
        None => {
            println!("Nothing found for double...")
        }
    }

    match find_triple(&lines) {
        Some((num1, num2, num3)) => {
            println!("{} * {} * {} = {}", num1, num2, num3, num1 * num2 * num3)
        }
        None => {
            println!("Nothing found for triple...")
        }
    }


    Ok(())
}
