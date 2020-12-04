// extern crate regex;
use regex::Regex;
use std::fs;

fn main() -> std::io::Result<()> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let _text = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";

    let lines = fs::read_to_string("input")?;

    let mut counter1 = 0;
    let mut counter2 = 0;

    for cap in re.captures_iter(lines.as_str()) {
        let min = cap[1].parse::<usize>().unwrap();
        let max = cap[2].parse::<usize>().unwrap();
        let c = cap[3].chars().nth(0).unwrap();
        let p = &cap[4];
        let count = p.matches(&cap[3]).count();

        println!(
            "Min: {} Max: {} Char: {} Password: {} Count: {}",
            min, max, c, p, count
        );

        if min <= count && count <= max {
            counter1 += 1;
        }

        match (p.chars().nth(min - 1), p.chars().nth(max - 1)) {
            (Some(a), Some(b)) if a == c && b == c => {}
            (Some(a), Some(b)) if a == c || b == c => {
                println!("Valid!");
                counter2 += 1;
            }
            _ => {}
        }
    }

    println!(">> challenge 1 valid passwords: {}", counter1);
    println!(">> challenge 2 valid passwords: {}", counter2);

    Ok(())
    /*
    // Output:
    let nums = fs::read_to_string("input1.txt")?
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        // .combinations(3)
        .filter(|c| c.iter().sum::<i32>() == 2020)
        .flatten()
        .collect::<Vec<i32>>();

    println!(">> a * b * c = {}", nums.iter().fold(1, |acc, x| acc * x));

    Ok(())
    */
}
