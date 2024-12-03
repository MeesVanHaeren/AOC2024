use std::fs;
use regex::Regex;

const FILEPATH: &str = "resources/day3Input.txt";

pub fn day3() {
    //get the entire inputstring
    let fileString = fs::read_to_string(FILEPATH).expect("Could not read file");
    //filter out all the mul operation with regex
    // mul\(\d+,\d+\)
    let muls = getMuls(fileString);
    //parse the mul operations into a pair of numbers
    let sum = getTotalFromMuls(muls);
    //interperet and sum up numbers
    println!("The sum of all muls: {}", sum);
}

fn getTotalFromMuls(muls: Vec<String>) -> i64 {
    let re = Regex::new(r"\d+").unwrap();
    let mut total: i64 = 0;
    for mul in muls {
        let mut numbers: Vec<u32> = Vec::new();
        for number in re.captures_iter(&mul) {
            numbers.push(
                number[0].parse().unwrap()
            )
        }
        total = total + (numbers.get(0).unwrap() * numbers.get(1).unwrap() ) as i64
    }
    total
}

fn getMuls(inputString: String) -> Vec<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut output : Vec<String> = Vec::new();
    for match_ in re.captures_iter(inputString.as_str()) {
        output.push(match_[0].parse().unwrap());
    }
    output
}