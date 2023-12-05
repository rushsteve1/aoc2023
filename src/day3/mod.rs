#[derive(Debug)]
struct Pos {
    col: usize,
    row: usize,
}

impl Pos {
    fn new(offset: usize, width: usize) -> Pos {
        Pos {
            col: offset % width,
            row: offset / width,
        }
    }

    fn is_adjacent(&self, other: &Pos) -> bool {
        self.col.abs_diff(other.col) <= 1 && self.row.abs_diff(other.row) <= 1
    }
}

#[derive(Debug)]
enum Entity {
    Sym { sym: char, pos: Pos },
    // max 3 digit numbers?
    Num { num: u32, start: Pos, end: Pos },
}

peg::parser! {
    grammar engine() for str {
        rule _ -> Option<Entity>
            = quiet!{ ['.' | ' ']+ } { None }

        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        rule posnum(width: usize) -> Option<Entity>
            = s:position!() n:number() e:position!()
        {
            Some(Entity::Num { num: n, start: Pos::new(s, width), end: Pos::new(e, width) })
        }

        rule sym() -> char
            // symbols other than period
            = ['!'..='-' | '/' | ':'..='@' | '['..='`' | '{'..='~']

        rule possym(width: usize) -> Option<Entity>
            = p:position!() c:sym()
        {
            Some(Entity::Sym { sym: c, pos: Pos::new(p, width)})
        }

        rule line(width: usize) -> Vec<Option<Entity>>
            = e:(possym(width) / posnum(width) / _)+

        pub rule main(width: usize) -> (Vec<Entity>, Vec<Entity>)
            = e:line(width) ** "\n"
        {
            // Due to a minor macro issue I've had to put more here than I'd like
            e.into_iter()
                .flatten()
                .filter_map(|e| e)
                .partition(|e| if let Entity::Sym{ sym: _, pos: _ } = e { true } else {false} )
        }
    }
}

pub fn part1<T: AsRef<str>>(input: T) -> u32 {
    let width = input.as_ref().lines().next().unwrap().len() + 1; // +1 for the \n
    let (symbols, numbers) = engine::main(input.as_ref(), width).unwrap();

    numbers.into_iter().filter_map(|n| {
        // I wish Rust had guard let like Swift does
        if let Entity::Num { num, start, end } = n {
            for s in &symbols {
                if let Entity::Sym { sym: _, pos } = s {
                    if start.is_adjacent(pos) || end.is_adjacent(pos) {
                        return Some(num);
                    }
                }
            }
        }
        return None;
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day_file;

    const EX: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1_test() {
        assert!(part1(EX) == 4361);
        assert!(dbg!(part1(day_file!())) == 0); // 512665 is too high
    }
}
