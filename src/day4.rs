use std::collections::HashSet;

use itertools::Itertools;

use crate::day::AocDay;

pub struct Day4;

impl Day4 {
    pub fn is_valid_checksum(input: &(String, u32, Vec<char>)) -> bool {
        let (name, sector_id, checksum) = input;
        let counts = name.chars()
            .filter(|c| *c != '-')
            .counts();
        let characters: Vec<_> = counts
        .iter()
        .sorted_by(|a, b| a.0.cmp(b.0))
        .sorted_by(|a, b| b.1.cmp(a.1))
        .map(|a| a.0)
        .take(5)
        .collect();
        characters.into_iter().collect::<HashSet<_>>() ==
        checksum.iter().collect::<HashSet<_>>()
    }

    pub fn decrypt(text: &str, sector_id: u32) -> String {
        text.chars().map(|c| {
            if c.is_alphabetic() {
                (((c as u32) - ('a' as u32) + sector_id) % 26 + ('a' as u32)) as u8 as char
            } else {
                c
            }
        }).collect()
    }
}

impl AocDay for Day4 {
    type I = Vec<(String, u32, Vec<char>)>;
    type O = u32;

    fn filename(&self) -> &'static str {
        "input/day4.txt"
    }

    fn parse(&self, contents: &str) -> Self::I {
        // "aaaaa-bbb-z-y-x-123[abxyz]"
        contents.lines().map(|line| {
            let (name, rest) = line.rsplit_once('-').unwrap();
            let (sector_id, checksum_raw) = rest.split_once('[').unwrap();
            (
                name.to_owned(), 
                sector_id.parse().unwrap(), 
                checksum_raw.chars()
                    .take(checksum_raw.len()-1)
                    .collect()
            )
        }).collect()
    }

    fn part1(&self, input: &Self::I) -> Self::O {
        input.iter()
            .filter(|&item| Self::is_valid_checksum(item))
            .map(|item| item.1)
            .sum()
    }

    fn part2(&self, input: &Self::I) -> Self::O {
        for entry in input {
            let decrypted = Self::decrypt(&entry.0, entry.1);
            if decrypted.contains("north") && decrypted.contains("pole") {
                println!("Found {decrypted}");
                return entry.1;
            }
        }
        println!("Not found");
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day4::Day4};

    #[test]
    fn test_part1() {
        let entries = [
            (true, "aaaaa-bbb-z-y-x-123[abxyz]"),
            (true, "a-b-c-d-e-f-g-h-987[abcde]"),
            (true, "not-a-real-room-404[oarel]"),
            (false, "totally-real-room-200[decoy]"),
        ];
        for entry in entries {
            let parsed = Day4.parse(entry.1);
            assert_eq!(entry.0, Day4::is_valid_checksum(&parsed[0]), "{:?}", entry);
        }
    }

    #[test]
    fn test_part2() {
        let entry = ("qzmt-zixmtkozy-ivhz".to_owned(), 343);
        assert_eq!(String::from("very-encrypted-name"), Day4::decrypt(&entry.0, entry.1))
    }
}