use std::fs;
use std::path::Component::ParentDir;

const FILEPATH: &str = "resources/day5Input.txt";

pub fn day5() {

    println!("Day 5");

    println!("total valid middle updates: {}", getMiddleUpdates() );

    print!("Total invalid-validated middle updates: {}", getInvalidMiddleUpdates());
}

fn getInvalidMiddleUpdates() -> u32 {
    let mut output = 0;

    let (order_tuple_vec, update_vecs) = parse_input();

    let invalid_updates: Vec<Vec<u32>> = update_vecs.iter().filter(|v| !update_is_valid(&order_tuple_vec, v)).cloned().collect();

    for invalid_update in invalid_updates {
        let valid_update = validify_update(&invalid_update,&order_tuple_vec);
        output = output + getMiddleUpdate(&valid_update);
    }

    output
}

fn validify_update(bad_update: &Vec<u32>, order_list: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut good_update = bad_update.clone();
    while !update_is_valid(&order_list, &good_update) {
        good_update = swap_invalid_rules(&good_update, &order_list);
    }
    good_update
}

fn swap_invalid_rules(bad_update: &Vec<u32>, order_list: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut good_update = bad_update.clone();
    for order in order_list {
        let mut encountered_first_value = false;
        let mut encountered_second_value = false;
        let mut second_value_index = 0;
        for i in 0..bad_update.len() {
            if order.0.eq(&bad_update[i]) && !encountered_second_value {
                encountered_first_value = true;
            } else if order.0.eq(&bad_update[i]) && encountered_second_value {
                good_update[i] = order.1;
                good_update[second_value_index] = order.0;
                break;
            } else if order.1.eq(&bad_update[i]) {
                encountered_second_value = true;
                second_value_index = i;
            }
        }
    }
    good_update
}

fn getMiddleUpdates() -> u64 {
    let mut output = 0;
    let (order_tuple_vec, update_vecs) = parse_input();

    let valid_updates: Vec<Vec<u32>> = update_vecs.iter().filter(|v| update_is_valid(&order_tuple_vec, v)).cloned().collect();

    for valid_update in valid_updates {
        output = output + (getMiddleUpdate(&valid_update) as u64)
    }

    output
}

fn parse_input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let inputString = fs::read_to_string(FILEPATH).expect("Could not read file");

    let first: &str = inputString.split("\n\n").collect::<Vec<&str>>()[0];

    let order_tuple_vec: Vec<(u32, u32)> = first.split("\n").map(get_tuple).collect();

    let last: &str = inputString.split("\n\n").collect::<Vec<&str>>()[1];

    let update_vecs: Vec<Vec<u32>> = last.split("\n").map(get_update_vec).collect();
    (order_tuple_vec, update_vecs)
}

fn update_is_valid(orderings: &Vec<(u32, u32)>, update: &Vec<u32>) -> bool {
    // println!("Checking update : {:?}", update);
    for ordering in orderings {
           if ordering_is_incompatible(ordering, &update){
               return false;
           }
   }

    true
}

fn ordering_is_incompatible(p0: &(u32, u32), p1: &Vec<u32>) -> bool {
    let mut encountered_first_value = false;
    let mut encountered_second_value = false;
    for number in p1 {
        if p0.0.eq(number) && !encountered_second_value {
            encountered_first_value = true;
        } else if p0.0.eq(number) && encountered_second_value {
            return true;
        } else if p0.1.eq(number) {
            encountered_second_value = true;
        }
    }
    false
}

fn get_tuple(input: &str) -> (u32, u32) {
    let vec = input.split("|").collect::<Vec<&str>>();

    (vec[0].parse::<u32>().unwrap(),vec[1].parse::<u32>().unwrap())
}

fn get_update_vec(input: &str) -> Vec<u32> {
    let mut output_vec: Vec<u32> = Vec::new();
    for number_string in input.split(",") {
        output_vec.push(number_string.parse::<u32>().unwrap());
    }
    output_vec
}

fn getMiddleUpdate(input: &Vec<u32>) -> u32 {
    input[input.len()/2]
}

fn tupleContains(input: &(u32,u32), number: &u32) -> bool {
    return input.0.eq(number) || input.1.eq(number)
}



fn contains_any_number(ordering: &Vec<(u32,u32)>, slice: &[u32], left_or_right: u32) -> bool {
    for number in slice {
        for tuple in ordering {
            if left_or_right == 0 {
                if tuple.0.eq(number) {
                    return true;
                }
            } else{
                if tuple.1.eq(number) {
                    return true;
                }
            }
        }
    }
    false
}