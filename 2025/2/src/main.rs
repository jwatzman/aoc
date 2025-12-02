use std::{env, fs};

fn num_digits(n: u64) -> u32 {
    let f: f64 = num_traits::cast(n).unwrap();
    num_traits::cast((f + 0.01).log10().ceil()).unwrap()
}

struct SillyIter {
    top: u64,
}

impl SillyIter {
    fn new(init: u64) -> Self {
        let num_digits = num_digits(init);

        if num_digits % 2 == 0 {
            let exp = num_digits / 2;
            let pow10 = 10_u64.pow(exp);

            let top = init / pow10;
            let bot = init % pow10;

            if top < bot {
                SillyIter { top: top + 1 }
            } else {
                SillyIter { top }
            }
        } else {
            let exp = (num_digits - 1) / 2;
            let pow10 = 10_u64.pow(exp);
            SillyIter { top: pow10 }
        }
    }
}

impl Iterator for SillyIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let exp = num_digits(self.top);
        let pow10 = 10_u64.pow(exp);

        let res = self.top * pow10 + self.top;

        self.top += 1;
        return Some(res);
    }
}

fn parse_line(line: &str) -> (u64, u64) {
    let mut split = line.split('-');
    let l = split.next().unwrap().parse().unwrap();
    let r = split.next().unwrap().parse().unwrap();

    return (l, r);
}

fn parse_input(contents: String) -> Vec<(u64, u64)> {
    contents
        .trim()
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| parse_line(s))
        .collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let input = parse_input(contents);

    let mut res = 0_u64;
    for (start, end) in input {
        let mut i = SillyIter::new(start);
        let mut n = i.next().unwrap();
        while n <= end {
            res += n;
            n = i.next().unwrap();
        }
    }

    println!("{}", res);
}
