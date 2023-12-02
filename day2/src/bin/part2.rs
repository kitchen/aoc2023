use regex::Regex;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum: u32 = 0;
    for line in stdin.lock().lines() {
        let real_line = line.unwrap();

        sum += power(&real_line);
    }

    println!("sum of game powers: {}", sum);
}

pub fn power(line: &str) -> u32 {
    let game_id_re = Regex::new(r"Game (?<game_id>\d+): (?<rest>.*)").unwrap();
    let draw_part_re = Regex::new(r"(?<count>\d+) (?<color>red|green|blue)").unwrap();

    println!("{}", line);
    let game_id_captures = game_id_re.captures(line).unwrap();

    let rest = game_id_captures.name("rest").unwrap().as_str();

    let mut min_red: u32 = 0;
    let mut min_green: u32 = 0;
    let mut min_blue: u32 = 0;
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
            match (color, count) {
                ("red", count) if count > min_red => min_red = count,
                ("green", count) if count > min_green => min_green = count,
                ("blue", count) if count > min_blue => min_blue = count,
                (_, _) => {}
            }
        }
    }

    return min_red * min_green * min_blue;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrived() {
        assert_eq!(
            48,
            power("Game 42: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")
        );
    }

    #[test]
    fn samples() {
        assert_eq!(
            48,
            power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",)
        );
        assert_eq!(
            12,
            power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",)
        );
        assert_eq!(
            1560,
            power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",)
        );
        assert_eq!(
            630,
            power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",)
        );
        assert_eq!(
            36,
            power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",)
        );
    }
}
