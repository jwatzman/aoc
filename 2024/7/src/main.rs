use std::env;
use std::fs;

fn parse_input(contents: String) -> Vec<(i64, Vec<i64>)> {
    let mut r = Vec::new();

    for line in contents.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let spl: Vec<_> = line.split(':').collect();
        if spl.len() != 2 {
            panic!("Invalid line: {line}");
        }

        let val: i64 = spl[0].parse().unwrap();
        let nums: Vec<i64> = spl[1]
            .trim()
            .split(' ')
            .map(|v| v.parse().unwrap())
            .rev()
            .collect();
        r.push((val, nums));
    }

    return r;
}

fn try_solve(val: i64, nums: &[i64]) -> bool {
    let n = nums[0];
    if nums.len() == 1 {
        return n == val;
    }

    let rest = &nums[1..];

    if val >= n {
        if try_solve(val - n, rest) {
            return true;
        }
    }

    if val % n == 0 {
        if try_solve(val / n, rest) {
            return true;
        }
    }

    let digits = n.ilog10() + 1;
    let adj = 10_i64.checked_pow(digits).unwrap();
    if val % adj == n {
        if try_solve(val / adj, rest) {
            return true;
        }
    }

    return false;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let eqns = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r = 0;
    for (val, nums) in &eqns {
        if try_solve(*val, nums) {
            r += val;
        }
    }

    println!("{r}");
}
