use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lock().lines() {
        let real_line = line.unwrap();

        sum += match is_game_possible(&real_line, 12, 13, 14) {
            (possible, id) if possible => id,
            _ => 0,
        };
    }

    println!("sum of game IDs: {}", sum);
}

// returns true or false and the game's ID
pub fn is_game_possible(line: &str, red: u32, green: u32, blue: u32) -> (bool, u32) {
    let game_id_re = Regex::new(r"Game (?<game_id>\d+): (?<rest>.*)").unwrap();
    let draw_part_re = Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").unwrap();

    println!("{}", line);
    let game_id_captures = game_id_re.captures(line).unwrap();
    let game_id: u32 = game_id_captures
        .name("game_id")
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    let rest = game_id_captures.name("rest").unwrap().as_str();

    for draw in rest.split(";") {
        // draw should look like: "2 red, 4 green, 5 blue"
        for color in draw.split(", ") {
            let color_matches = draw_part_re.captures(color).unwrap();
            let count: u32 = color_matches
                .name("count")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let color = color_matches.name("color").unwrap().as_str();

            if (color == "red" && count > red)
                || (color == "green" && count > green)
                || (color == "blue" && count > blue)
            {
                return (false, game_id);
            }
        }
    }

    return (true, game_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrived() {
        assert_eq!(
            (false, 42),
            is_game_possible(
                "Game 42: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                1,
                1,
                1
            )
        );
    }

    #[test]
    fn samples() {
        assert_eq!(
            (true, 1),
            is_game_possible(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                12,
                13,
                14
            )
        );
        assert_eq!(
            (true, 2),
            is_game_possible(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
                13,
                14
            )
        );
        assert_eq!(
            (false, 3),
            is_game_possible(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                12,
                13,
                14
            )
        );
        assert_eq!(
            (false, 4),
            is_game_possible(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                12,
                13,
                14
            )
        );
        assert_eq!(
            (true, 5),
            is_game_possible(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                12,
                13,
                14
            )
        );
    }
}
