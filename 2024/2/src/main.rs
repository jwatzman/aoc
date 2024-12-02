use std::env;
use std::fs;

fn parse(contents: &String) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    for line in contents.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut levels = Vec::new();
        for s in line.split_ascii_whitespace() {
            if s.len() == 0 {
                continue;
            }

            levels.push(s.parse().expect("Expected a number"));
        }

        reports.push(levels);
    }

    return reports;
}

fn report_safe(report: &Vec<i32>) -> bool {
    let mut prev = None;
    let mut increasing = None;
    for level in report {
        match prev {
            None => (),
            Some(prev) => {
                let diff: i32 = level - prev;
                if diff == 0 || diff.abs() > 3 {
                    return false;
                }

                let now_increasing = diff > 0;
                match increasing {
                    None => increasing = Some(now_increasing),
                    Some(increasing) => {
                        if increasing != now_increasing {
                            return false;
                        }
                    }
                }
            }
        }

        prev = Some(level);
    }

    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");

    let reports = parse(&contents);
    let mut res = 0;
    for report in reports {
        if report_safe(&report) {
            res += 1;
        }
    }

    println!("{res}");
}
