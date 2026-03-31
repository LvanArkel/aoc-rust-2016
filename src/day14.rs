use std::{collections::HashMap, fmt::format};

use itertools::{Itertools, repeat_n};

use crate::day::AocDay;

pub struct Day14;

fn has_triple(input: &str) -> Option<char> {
    match input.chars().tuple_windows().find(|(a, b, c)| {
        a == b && a == c
    }) {
        Some((c,_, _)) => Some(c),
        None => None,
    }
}

fn has_subsequence(input: &str, sequence: &str) -> bool {
    input.contains(sequence)
}

fn hash(
    salt: &str, 
    index: usize,
    cache: &mut HashMap<usize, String>,
) -> String {
    match cache.get(&index) {
        Some(hash) => hash.to_owned(),
        None => {
            let key = format!("{salt}{index}");
            let hash = md5::compute(key);
            let hash_text = format!("{hash:x}");
            cache.insert(index, hash_text.clone());
            hash_text
        },
    }
}

fn find_next_index_p1(salt: &str, i: usize) -> usize {
    let mut index = i;
    let mut cache: HashMap<_, _> = HashMap::new();
    loop {
        let h = hash(salt, index, &mut cache);
        if let Some(c) = has_triple(&h) {
            let subsequence: String = repeat_n(c, 5).collect();
            for j in 1..=1000 {
                let h2 = hash(salt, index + j, &mut cache);
                if has_subsequence(&h2, &subsequence) {
                    return index;
                }
            }
        }
        index += 1;
    }
}

fn hash_p2(
    salt: &str, 
    index: usize,
    cache: &mut HashMap<usize, String>,
) -> String {
    match cache.get(&index) {
        Some(hash) => hash.to_owned(),
        None => {
            let mut key: String = format!("{salt}{index}");
            for _ in 0..=2016 {
                let hash = md5::compute(key);
                key = format!("{hash:x}");
            }
            cache.insert(index, key.clone());
            key
        },
    }
}

fn find_next_index_p2(salt: &str, i: usize) -> usize {
    let mut index = i;
    let mut cache: HashMap<_, _> = HashMap::new();
    loop {
        let h = hash_p2(salt, index, &mut cache);
        if let Some(c) = has_triple(&h) {
            let subsequence: String = repeat_n(c, 5).collect();
            for j in 1..=1000 {
                let h2 = hash_p2(salt, index + j, &mut cache);
                if has_subsequence(&h2, &subsequence) {
                    return index;
                }
            }
        }
        index += 1;
    }
}


impl AocDay for Day14 {
    type I = String;

    type O = usize;

    fn filename() -> &'static str {
        "input/day14.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.to_owned()
    }

    fn part1(input: &Self::I) -> Self::O {
        let mut i = 0;
        for _ in 0..64 {
            i = find_next_index_p1(input, i+1);
        }
        i
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut i = 0;
        for n in 1..=64 {
            i = find_next_index_p2(input, i+1);
            println!("Found {n}={i}");
        }
        i
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{day::AocDay, day14::{Day14, hash_p2}};

    #[test]
    fn test_part1() {
        assert_eq!(22728, Day14::part1(&"abc".to_owned()));
    }

    #[test]
    fn test_part2_hash() {
        assert_eq!("a107ff634856bb300138cac6568c0f24", hash_p2("abc", 0, &mut HashMap::new()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(22551, Day14::part2(&"abc".to_owned()))
    }
}