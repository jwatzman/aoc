use std::collections::HashMap;
use std::env;
use std::fs;

type WireName<'a> = &'a str;

#[derive(Debug, PartialEq, Eq)]
enum WireState {
    Value(bool),
    NoValue,
}

#[derive(Debug)]
enum GateOp {
    And,
    Or,
    Xor,
}

#[derive(Debug)]
struct Gate<'a> {
    op: GateOp,
    input: (WireName<'a>, WireName<'a>),
    output: WireName<'a>,
}

fn parse_input(contents: &String) -> (HashMap<WireName, WireState>, Vec<Gate>) {
    let mut it = contents.split('\n');

    let mut wires = HashMap::new();
    loop {
        let line = it.next().unwrap();
        if line.is_empty() {
            break;
        }

        let wire_name = &line[..3];
        let wire_bool = match line.chars().last().unwrap() {
            '0' => false,
            '1' => true,
            _ => panic!(),
        };

        wires.insert(wire_name, WireState::Value(wire_bool));
    }

    let mut gates = Vec::new();
    loop {
        let line = it.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut spl = line.split(' ');
        let input1 = spl.next().unwrap();
        let op = match spl.next().unwrap() {
            "AND" => GateOp::And,
            "OR" => GateOp::Or,
            "XOR" => GateOp::Xor,
            _ => panic!(),
        };
        let input2 = spl.next().unwrap();
        spl.next();
        let output = spl.next().unwrap();

        wires.entry(input1).or_insert(WireState::NoValue);
        wires.entry(input2).or_insert(WireState::NoValue);
        wires.entry(output).or_insert(WireState::NoValue);

        gates.push(Gate {
            op,
            input: (input1, input2),
            output,
        });
    }

    return (wires, gates);
}

fn overwrite_input(wires: &mut HashMap<WireName, WireState>, prefix: char, mut n: u64) {
    let mut i = 0_u64;
    loop {
        let wire_name = format!("{prefix}{i:02}");
        let wire_ref = match wires.get_mut(wire_name.as_str()) {
            None => {
                assert_eq!(n, 0);
                break;
            }
            Some(r) => r,
        };

        assert_ne!(*wire_ref, WireState::NoValue);
        *wire_ref = WireState::Value((n & 1) == 1);

        i += 1;
        n >>= 1;
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();
    let (mut wires, mut gates) = parse_input(&contents);

    let overwrites = if args.len() == 4 {
        let x = args[2].parse().unwrap();
        overwrite_input(&mut wires, 'x', x);
        let y = args[3].parse().unwrap();
        overwrite_input(&mut wires, 'y', y);
        Some((x, y))
    } else {
        None
    };

    loop {
        let old_len = gates.len();

        gates.retain(|gate| {
            let input1_state = wires.get(gate.input.0).unwrap();
            let input2_state = wires.get(gate.input.1).unwrap();

            let (input1, input2) = match (input1_state, input2_state) {
                (WireState::Value(a), WireState::Value(b)) => (*a, *b),
                _ => return true,
            };

            let output_state = wires.get_mut(gate.output).unwrap();
            if *output_state != WireState::NoValue {
                return true;
            }

            let output = match gate.op {
                GateOp::And => input1 && input2,
                GateOp::Or => input1 || input2,
                GateOp::Xor => input1 ^ input2,
            };

            *output_state = WireState::Value(output);
            return false;
        });

        if gates.len() == old_len {
            break;
        }
    }

    let mut r = 0_u64;
    let mut i = 0_u64;
    loop {
        let wire_name = format!("z{i:02}");
        match wires.get(wire_name.as_str()) {
            None => break,
            Some(WireState::NoValue) => panic!(),
            Some(WireState::Value(true)) => r |= 1 << i,
            Some(WireState::Value(false)) => (),
        }

        i += 1;
    }

    println!("{r}");

    match overwrites {
        None => (),
        Some((x, y)) => {
            let mut i = 0_u64;
            loop {
                let wire_name = format!("z{i:02}");
                let c = match wires.get(wire_name.as_str()) {
                    None => break,
                    Some(WireState::NoValue) => panic!(),
                    Some(WireState::Value(true)) => '1',
                    Some(WireState::Value(false)) => '0',
                };
                println!("{wire_name}: {c}");

                i += 1;
            }

            println!("");
            println!("{x:#048b}");
            println!("{y:#048b}");
            println!("{r:#048b}");
        }
    }
}
