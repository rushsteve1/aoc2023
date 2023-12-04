const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

#[derive(Debug, Default)]
struct Frame {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    index: u32,
    frames: Vec<Frame>,
}

impl Game {
    fn valid(&self) -> bool {
        for f in &self.frames {
            if f.red > RED_MAX || f.green > GREEN_MAX || f.blue > BLUE_MAX {
                return false;
            }
        }

        return true;
    }

    fn mins(&self) -> (u32, u32, u32) {
        let (mut r, mut g, mut b) = (0, 0, 0);
        for f in &self.frames {
            r = r.max(f.red);
            g = g.max(f.green);
            b = b.max(f.blue);
        }
        (r, g, b)
    }
}

peg::parser! {
    grammar game_parser() for str {
        rule _() = " " / "\t"

        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        rule color() -> (&'input str, u32)
            = n:number() _ c:$("red" / "green" / "blue") { (c, n) }

        rule frame() -> Frame
            = c:(color() ++ ", ") {
                let mut f = Frame::default();
                for (c, n) in c {
                    match c {
                        "red" => { f.red = n }
                        "green" => { f.green = n}
                        "blue" => { f.blue = n}
                        _ => unreachable!()
                    }
                }
                f
             }

        rule game() -> Game
            = "Game " i:number() ": " f:(frame() ** "; ") { Game { index: i, frames: f } }

        pub rule all_games() -> Vec<Game>
            = game() ** "\n"
    }
}

pub fn part1<T: AsRef<str>>(input: T) -> u32 {
    let parsed = game_parser::all_games(input.as_ref()).unwrap();

    parsed
        .into_iter()
        .filter_map(|g| if g.valid() { Some(g.index) } else { None })
        .sum()
}

pub fn part2<T: AsRef<str>>(input: T) -> u32 {
    let parsed = game_parser::all_games(input.as_ref()).unwrap();

    parsed
        .into_iter()
        .map(|g| g.mins())
        .map(|(r, g, b)| r * g * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_file;

    const EX: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1_test() {
        assert!(part1(EX) == 8);
        assert!(part1(day_file!()) == 2105);
    }

    #[test]
    fn part2_test() {
        assert!(part2(EX) == 2286);
        assert!(part2(day_file!()) == 72422);
    }
}
