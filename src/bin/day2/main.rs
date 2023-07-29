use itertools::Itertools;
use std::collections::HashMap;

fn apply_direction(instruction: char, current_position: [i32; 2]) -> [i32; 2] {
    return match instruction {
        'L' => [current_position[0], current_position[1] - 1],
        'R' => [current_position[0], current_position[1] + 1],
        'U' => [current_position[0] - 1, current_position[1]],
        'D' => [current_position[0] + 1, current_position[1]],
        char => panic!("incorrect input : {}", char)
    }
}

fn main() {
    let start_place = [2, 0];
    let key_pad = HashMap::from([([0,2], "1"), ([1,1], "2"), ([1,2], "3"), ([1,3], "4"), ([2,0], "5"), ([2,1], "6"), ([2,2], "7"), ([2,3], "8"), ([2,4], "9"), ([3,1], "A"), ([3,2], "B"), ([3,3], "C"), ([4,2], "D")]);
    
    let code = include_str!("input.txt")
        .lines()
        .map(|lines| {            
            let digit_position = lines.chars().fold(start_place, |current_position, direction| {
                let new_position = apply_direction(direction,current_position);
                if key_pad.contains_key(&new_position) { new_position } else { current_position }
            });
        
            return key_pad.get(&digit_position).expect("We should always have this position");
        })
        .join("");

    println!("{}", code);
}
