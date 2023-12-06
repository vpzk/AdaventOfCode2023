use std::{ fs::read_to_string};
use regex::Regex;

const FILE: &str = "./src/input.txt";

#[derive(Debug)]
struct Game {
    id: i16,
    points: u32,
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
       result.push(line.to_string())
    }
    result
}

fn make_game(line: String) -> Game {

    let mut game = Game {
        id: 0,
        points: 0
    };
    println!("line: {:?}", line);
    let id_card_pattern = Regex::new(r"Card (?<id>\d+)").unwrap();
    match id_card_pattern.captures(&line) {
        Some(capture) => {
            if let Some(id_match) = capture.get(1) {
                let id = id_match.as_str();
                println!("id : {}", id);
                game.id = id.parse::<i16>().unwrap();
            }
        },
        None => println!("No match")
    }
    let clean_line: Vec<_> = line.split(": ").nth(1).unwrap().split("| ").collect();
    let wins_string: Vec<_> = clean_line[0].split_whitespace().collect();
    let wins_num: Vec<i16> = wins_string.into_iter()
        .filter_map(|num| num.parse::<i16>().ok())
        .collect();
    println!("wins_num: {:?}", wins_num);

    let my_nums_string: Vec<_> = clean_line[1].split_whitespace().collect();
    let my_nums: Vec<i16> = my_nums_string.into_iter()
        .filter_map(|num| num.parse::<i16>().ok())
        .collect();
    println!("my_nums: {:?}", my_nums);
    for win in wins_num {
        if my_nums.contains(&win) {game.points += 1;}
    }
    println!("Game: {:?}", game);
    game
}

fn main() {
    let lines = read_lines(FILE);
    let mut result: u32 = 0;
    for line in lines {
        let res = make_game(line).points;
        if(res > 1) {result +=1 * u32::pow(2, res -1)}
        else {result += res }
    }
    println!("result :{:?}", result);
}
