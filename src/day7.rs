use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;

use crate::day::AocDay;

pub struct Day7;

impl AocDay for Day7 {
    type I = Vec<String>;

    type O = usize;

    fn filename(&self) -> &'static str {
        "input/day7.txt"
    }

    fn parse(&self, contents: &str) -> Self::I {
        contents.lines().map(|line| {
            line.to_owned()
        }).collect()
    }

    fn part1(&self, input: &Self::I) -> Self::O {
        input.iter()
            .filter(|&line | supports_tls(line))
            .count()
    }

    fn part2(&self, input: &Self::I) -> Self::O {
        input.iter()
            .filter(|&line| supports_ssl(line))
            .count()
    }
}

static pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"(.)(?!\1)(.)\2\1"#).unwrap());
fn supports_tls(input: &str) -> bool {
    let lbrs: Vec<_> = input.char_indices()
        .filter(|(i, c)| *c == '[')
        .map(|(i, c)| i)
        .collect();
    let rbrs: Vec<_> = input.char_indices()
        .filter(|(i, c)| *c == ']')
        .map(|(i, c)| i)
        .collect();
    let brackets: Vec<_> = lbrs.into_iter().zip(rbrs).collect();
    let matches: Vec<_> = pattern.find_iter(input).collect();
    if matches.is_empty() {
        return false;
    }
    matches.into_iter().all(|m| {
        let start = m.unwrap().start();
        brackets.iter().all(|(b_start, b_end)| !(*b_start..*b_end).contains(&start))
    })    
}

static group_pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\w+"#).unwrap());
fn supports_ssl(input: &str) -> bool {
    let groups: Vec<_> = group_pattern
        .find_iter(input)
        .map(|m| m.unwrap().as_str())
        .collect();
    let (supernet, hypernet): (Vec<_>, Vec<_>) = groups
    .into_iter()
    .enumerate()
    .partition_map(|(i, text)| {
        if i % 2 == 0  {
            itertools::Either::Left(text)
        } else {
            itertools::Either::Right(text)
        }
    });
    supernet
        .into_iter()
        .flat_map(|segment| {
            (0..segment.len() - 2).map(|i| {
                &segment[i..i+3]
            })
        })
        .filter(|&segment| {
            let chars: Vec<_> = segment.chars().collect();
            chars[0] == chars[2] && chars[0] != chars[1]
        })
        .any(|aba| {
            let chars: Vec<_> = aba.chars().collect();
            let bab: String = vec![chars[1], chars[0], chars[1]].iter().collect();
            hypernet.iter().any(|segment| segment.contains(&bab))
        })
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day7::{Day7, supports_ssl, supports_tls}};

    #[test]
    fn test_supports_tls() {
        let day = Day7;
        assert!(supports_tls(&"abba[mnop]qrst"));
        assert!(!supports_tls(&"abcd[bddb]xyyx"));
        assert!(!supports_tls(&"aaaa[qwer]tyui"));
        assert!(supports_tls(&"ioxxoj[asdfgh]zxcvbn"));
    }

    #[test]
    fn test_supports_ssl() {
        let day = Day7;
        assert!(supports_ssl(&"aba[bab]xyz"));
        assert!(!supports_ssl(&"xyx[xyx]xyx"));
        assert!(supports_ssl(&"aaa[kek]eke"));
        assert!(supports_ssl(&"zazbz[bzb]cdb"));
    }
}
