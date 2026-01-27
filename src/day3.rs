use crate::day::AocDay;

pub struct Day3;

impl Day3 {
    fn is_triangle(triangle: &(u32, u32, u32)) -> bool {
        triangle.0 + triangle.1 > triangle.2 &&
        triangle.1 + triangle.2 > triangle.0 &&
        triangle.2 + triangle.0 > triangle.1
    }
}

impl AocDay for Day3 {
    type I = Vec<(u32, u32, u32)>;
    type O = usize;
    
    fn filename(&self) -> &'static str {
        "input/day3.txt"
    }
    
    fn parse(&self, contents: &str) -> Self::I {
        contents.lines().map(|line| {
            let digits: Vec<_> = line.trim().split_ascii_whitespace().collect();
            assert_eq!(3, digits.len(), "{digits:?}");
            (
                digits[0].parse().unwrap(),
                digits[1].parse().unwrap(),
                digits[2].parse().unwrap(),
            )
        }).collect()
    }
    
    fn part1(&self, input: &Self::I) -> Self::O {
        input.iter().filter(|&tri| Day3::is_triangle(tri)).count()
    }
    
    fn part2(&self, input: &Self::I) -> Self::O {
        input.chunks(3).flat_map(|tris| {
            [
                (tris[0].0, tris[1].0, tris[2].0),
                (tris[0].1, tris[1].1, tris[2].1),
                (tris[0].2, tris[1].2, tris[2].2)
            ]
        }).filter(|tri| Day3::is_triangle(tri)).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::Day3;

    #[test]
    fn test_part1() {
        assert!(!Day3::is_triangle(&(5, 10, 25)))
    }
}
