const FILEPATH: &str = "resources/day1Input.txt";

use std::fs;
use std::io::BufRead;

pub fn day1() {
    //controller file
    println!("Welcome to day 1");

    println!("The difference is {}", difference());

    println!("The similarity is {}", similarity());
}

fn similarity() -> String {
    //read file to let string
    let fileString = fs::read_to_string(FILEPATH).expect("Could not read file");
    //split string into 2 vectors
    let rowVectors = splitRows(splitInputString(fileString));
    let mut left : Vec<u32> = parseVector(rowVectors.0);
    let mut right : Vec<u32> = parseVector(rowVectors.1);
    //get similarity score vector
    let simScoreVect = similarityVector(left,right);
    //get sum and return
    let sum: u32 = simScoreVect.iter().sum();
    sum.to_string()
}


fn difference() -> String {
    //read file to let string
    let fileString = fs::read_to_string(FILEPATH).expect("Could not read file");
    //split string into 2 vectors
    let rowVectors = splitRows(splitInputString(fileString));
    let mut left : Vec<u32> = parseVector(rowVectors.0);
    let mut right : Vec<u32> = parseVector(rowVectors.1);
    //sort the vectors
    left.sort();
    right.sort();
    //compare the vectors into third vector
    let diffVec = compareVectors(&left,&right);
    //Add up the vector into number
    let sum: u32 = diffVec.iter().sum();
    //print number
    sum.to_string()
}

fn splitInputString(input: String) -> Vec<String>{
    input.split("\n").map(String::from).collect()
}

fn splitRows(input: Vec<String>) -> (Vec<String>,Vec<String>){
    let mut left: Vec<String> = Vec::new();
    let mut right: Vec<String> = Vec::new();

    for row in input {
        let row_str: &str = &*row;
        let parts = row_str.split("   ").collect::<Vec<&str>>();
        left.push(String::from(parts[0]));
        right.push(String::from(parts[1]));
    }
    (left,right)
}

fn parseVector(input: Vec<String>) -> Vec<u32>{
    let mut output: Vec<u32> = Vec::new();
    for string in input{
        output.push(string.parse::<u32>().unwrap());
    }
    output
}

fn compareVectors(left: &Vec<u32>, right: &Vec<u32>) -> Vec<u32> {
    let mut output: Vec<u32> = Vec::new();

    for i in 0..left.len() {
        let leftNum = left[i];
        let rightNum = right[i];
        if leftNum < rightNum {
            output.push(rightNum - leftNum);
        } else {
            output.push(leftNum - rightNum);
        }
    }

    output
}

fn count(number: &u32, vector: &Vec<u32>) -> u32 {
    let mut count = 0;
    for vecNum in vector {
        if vecNum == number {
            count += 1;
        }
    }
    count
}

fn getSimilarityScoreMult (number: u32, vector: &Vec<u32>) -> u32 {
    &number * count(&number, vector)
}

fn similarityVector(left: Vec<u32>, right: Vec<u32>) -> Vec<u32> {
    let mut output : Vec<u32> = Vec::new();
    for number in left {
        output.push(getSimilarityScoreMult(number,&right));
    }
    output
}
