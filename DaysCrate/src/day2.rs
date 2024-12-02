const FILEPATH: &str = "resources/day2Input.txt";

use std::fs;
use std::io::BufRead;
use std::ptr::read;
use crate::day1::parseVector;

pub fn day2() {
    println!("Welcome to day 2");

    //controller function
    let input = inputGetter(FILEPATH.parse().unwrap());

    println!("input length: {}", &input.len());

    let safe = countSafe(input);
    println!("amount safe: {}", safe);
}

fn inputGetter(filePath: String) -> Vec<String> {
    let fileString = fs::read_to_string(filePath).expect("Could not read file");
    fileString.split("\n").map(String::from).collect()
}

fn countSafe(inputVec: Vec<String>) -> u32 {
    let mut amount = 0;
    for row in inputVec {
        let splitRow = row.split(" ").map(String::from).collect();
        if validateSafety(parseVector(splitRow)) {
            amount += 1;
        }
    }
    amount
}

fn validateSafety(temperatures: Vec<u32>) -> bool {
    if validateSafeSteady(&temperatures) && validateSafeSlow(&temperatures) {
        println!("I think this one is alright: {:?}", temperatures);
        return true;
    }
    false
    // validateSafeSteady(&temperatures) && validateSafeSlow(&temperatures)
}

fn validateSafeSteady(temperatures: &[u32]) -> bool {
    let mut safety: bool = true;

    let mut rising = false;
    if temperatures.get(0) < temperatures.get(1) {
        rising = true;
    }

    let mut prev = temperatures.get(0).unwrap();
    let mut otherTemps = &temperatures[1..temperatures.len()];

    for temp in otherTemps {

        if temp == prev {
            safety = false;
            break;
        }

        if rising {
            if temp < prev {
                safety = false;
                break;
            }
        } else {
            if temp > prev {
                safety = false;
                break;
            }
        }
        prev = temp;
    }
    safety
}

fn validateSafeSlow(temperatures: &[u32]) -> bool {
    let mut safety: bool = true;
    let mut prev= temperatures.get(0).unwrap();

    //this is a hack because I have no time. All of these for loops will iterate one useless time
    for temp in temperatures  {
        if getDiff(&temp, &prev) > 3 {
            safety = false;
            break;
        }
        prev = temp;
    }

    safety
}

fn getDiff (left : &u32, right : &u32) -> u32 {
    if left > right {
        return left - right;
    } else if right > left{
        return right - left;
    }
    return 0;
}