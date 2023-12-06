use std::{fs::read_to_string};
use regex::Regex;

const FILE: &str = "./src/input";

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

#[derive(Debug)]
struct Set {
    red: i16,
    green: i16,
    blue: i16
}

#[derive(Debug)]
struct Game {
    id: i16,
    sets: Vec<Set>,
    possible: bool
}

fn make_sets(game_string: &str) -> Set {
    let mut set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    let processed_string = if game_string.contains("Game") {
        game_string.split(": ").nth(1).unwrap_or(game_string)
    } else {
        game_string
    };

    println!("game clean issue: {:?}", processed_string);
    
    for item in processed_string.split(", ") {
        let parts: Vec<&str> = item.split_whitespace().collect();
        match parts[1] {
            "red" => set.red = parts[0].parse::<i16>().unwrap(),
            "green" => set.green = parts[0].parse::<i16>().unwrap(),
            "blue" => set.blue = parts[0].parse::<i16>().unwrap(),
            _ => {}
        }
    }
    set
}

fn make_game(string_game: String) -> Game {
    let split_string_game: Vec<_> = string_game.split("; ").collect();
    let id_game_pattern = Regex::new(r"Game (?<id>\d+)").unwrap();
    
    let mut game = Game {
        id: 0,
        sets: Vec::new(),
        possible: true,
    };
    for game_string in split_string_game {
        if game_string.contains("Game") {
            match id_game_pattern.captures(game_string) {
                Some(capture) => {
                    if let Some(id_match) = capture.get(1) {
                        let id = id_match.as_str();
                        println!("id : {}", id);
                        game.id = id.parse::<i16>().unwrap();
                    }
                },
                None => println!("No match")
            }
        }
        game.sets.push(make_sets(game_string));        
    }
    for set in &game.sets {
        if set.red > RULES.red || set.blue > RULES.blue || set.green > RULES.green{game.possible = false;}
    }
    println!("Game: {:?}", game);
    game
}

const RULES: Set = Set {
    red: 12,
    green: 13,
    blue: 14,
};

fn main() {
    let lines = read_lines(FILE);
    println!("{:?}",lines);
    let mut result: i16 = 0;
    for line in lines {
        let game = make_game(line);
        result += if game.possible {game.id} else {0};
        println!("Result :{:?}", result);
    }
}
