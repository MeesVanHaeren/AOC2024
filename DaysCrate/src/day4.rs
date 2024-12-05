use std::fs;

const FILEPATH: &str = "resources/day4InputSmall.txt";
//in small:
//5 hor V
//2 ver V
//7 dia ?
//total of 14


pub fn day4() {
    //get Input
    let input_string = fs::read_to_string(FILEPATH).expect("Could not read file");
    let input_vec = import2d_vec(input_string);

    //iterate over all
    let count = getAllXmases(&input_vec);

    //print
    println!("welcome to day 4 babe");
    println!("Amount of xmases horizontally, vertically, diagonally and in reverse: {}", count);
}

fn getAllXmases(vecs: &Vec<Vec<char>>) -> u32 {
    let mut output = 0;

    for row in vecs {
        let horizontal = getHorizontalXmases(row);
        output += horizontal;
        println!("hor {}", horizontal);
    }

    for columnIndex in 0..vecs[0].len() {
        let vertical = getVerticalXmases(vecs, columnIndex);
        output += vertical;
        println!("ver {}", vertical);
    }

    let dia = getDiagonalXmases(vecs);
    output += dia;
    println!("dia {}", dia);

    output
}

fn getVerticalXmases(vecs: &Vec<Vec<char>>, column: usize) -> u32 {
    let mut column_array: Vec<char> = Vec::new();

    for i in 0..vecs.len() {
        column_array.push(vecs[i][column]);
    }

    getHorizontalXmases(&column_array)
}

fn getHorizontalXmases(p0: &Vec<char>) -> u32 {
    let mut output = 0;
    let mut valid_slice = &p0[0..p0.len()-4];
    for i in 0..valid_slice.len() {
        if isXmas(&p0[i..i+4]){
            output += 1
        }
    }

    output
}

fn getDiagonalXmases(p0: &Vec<Vec<char>>) -> u32 {
    let mut output = 0;

    for y_i in 4..p0.len() {

        for x_i in 4..p0[y_i].len(){

            let mut temporary_array_nw_se = Vec::new();
            temporary_array_nw_se.push(p0[y_i-3][x_i-3]);
            temporary_array_nw_se.push(p0[y_i-2][x_i-2]);
            temporary_array_nw_se.push(p0[y_i-1][x_i-1]);
            temporary_array_nw_se.push(p0[y_i][x_i]);

            if isXmas(temporary_array_nw_se.as_slice()){
                output += 1
            }

            let mut temporary_array_ne_sw = Vec::new();
            temporary_array_ne_sw.push(p0[y_i-3][x_i]);
            temporary_array_ne_sw.push(p0[y_i-2][x_i-1]);
            temporary_array_ne_sw.push(p0[y_i-1][x_i-2]);
            temporary_array_ne_sw.push(p0[y_i][x_i-3]);

            if isXmas(temporary_array_ne_sw.as_slice()){
                output += 1
            }

        }

    }



    output
}

fn isXmas(p0: &[char]) -> bool {
    if p0.len() == 4 {
        return (p0[0] == 'X' && p0[1] == 'M' && p0[2] == 'A' && p0[3] == 'S')||(p0[0] == 'S' && p0[1] == 'A' && p0[2] == 'M' && p0[3] == 'X')
    }
    false
}

fn import2d_vec(input : String) -> Vec<Vec<char>>{
    let row_vec: Vec<String> = input.split('\n').map(String::from).collect();
    let mut output: Vec<Vec<char>> = Vec::new();

    for row in row_vec {
        output.push(row.chars().collect());
    }

    output
}