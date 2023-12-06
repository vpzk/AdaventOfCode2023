use std::{fs::read_to_string, ops::Add};
use serde_json::{Value, json, to_value};
use serde::{Serialize, Deserialize};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn get_digit(line: &str) -> String {
    let base = 10;
    let mut new_line = String::new();

    for ch in line.chars() {
        if ch.is_digit(base) {new_line.push(ch)};
    }
    new_line
}

fn get_first_and_last_digit(line: &str) -> String {
    let mut new_line = String::new();
    new_line.push(line.chars().next().unwrap());
    new_line.push(line.chars().last().unwrap());
    new_line
}

#[derive(Serialize, Deserialize)]
struct VecNumber {
    one: i32,
    two: i32,
    three: i32,
    four: i32,
    five: i32,
    six: i32,
    seven: i32,
    eight: i32,
    nine: i32,
}

fn transform_string_number_to_char(line: &str) -> String {
    let mut sentence = String::new();
    sentence.push_str(line);

    let vec_number = VecNumber {
        one: 1,
        two: 2,
        three: 3,
        four: 4,
        five: 5,
        six: 6,
        seven: 7,
        eight: 8,
        nine: 9,
    };

    let serialized = to_value(&vec_number).unwrap();

    if let Value::Object(map) = serialized {
        for (key, value) in map.iter() {
            if line.contains(key) {
                let value_str = value.as_str().unwrap();
                sentence.push_str(value_str);
            }
        }
    }

    sentence
}

fn main () {
    let lines = read_lines("./input");
    let mut result = Vec::new();
    for line in lines.iter() {
        let res1 = transform_string_number_to_char(line);
        println!("{:?}", res1);
        let res = get_digit(&res1);
        result.push(res);
    }

    let mut result2 = 0;
    for x in result.iter() {
        result2 += get_first_and_last_digit(x).parse::<i32>().unwrap();
    }
    println!("{:?}",result2)
}