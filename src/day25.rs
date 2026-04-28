use crate::day::AocDay;

pub struct Day25;

impl Day25 {
    fn run_machine(x: i32) {
        let max_prints = 40;
        let mut prints = 0;

        let mut a = x;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;
        
        // Line 1-2
        d = a;
        c = 4;

        // Line 3-8
        d += 633 * c;
        println!("{d} {d:b}");

        loop {
            a = d;

            while a != 0  {
                b = a % 2;
                a = a / 2;

                print!("{b}");
                prints += 1;
                if prints >= max_prints {
                    return;
                }
            }
        }
    }

    pub fn run() {
        println!("{}", 0b11000110);
        Self::run_machine(0b11000110);

    }
}