pub fn part1<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u32 {
    lines
        .map(|s| {
            let mut d = s.as_ref().chars().filter(|c| c.is_digit(10));
            let f = d.next();
            let l = d.next_back().or(f);

            f.zip(l)
                .and_then(|(f, l)| f.to_digit(10).zip(l.to_digit(10)))
                .map(|(f, l)| (f * 10) + l)
                .unwrap_or_default()
        })
        .sum()
}

pub fn part2<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> u32 {
    // This is lazy but very effective
    let mut nums = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("oneight", "18"),
        ("twone", "21"),
        ("threeight", "38"),
        ("fiveight", "58"),
        ("sevenine", "79"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("nineight", "98"),
    ];
    nums.sort_by(|(ak, _), (bk, _)| bk.len().cmp(&ak.len()));
    dbg!(&nums);

    let l = lines.map(|s| {
        nums.iter()
            .fold(s.as_ref().to_string(), |a, (k, v)| a.replace(k, v))
    });

    part1(l)
}

#[test]
fn part1_test() {
    use crate::day_file;

    let ex = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert!(dbg!(part1(ex.lines())) == 142);

    assert!(part1(day_file!("input.txt").lines()) == 54331);
}

#[test]
fn part2_test() {
    use crate::day_file;

    let ex = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert!(part2(ex.lines()) == 281);

    assert!(part2(day_file!("input.txt").lines()) == 54518);
}
