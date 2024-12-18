use rayon::prelude::*;
use regex::Regex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

advent_of_code::solution!(17);

#[derive(Default, Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
}

fn simulate(opcodes: &Vec<i64>, cpu: &mut Computer) -> String {
    let mut pc = 0;
    let mut out: Vec<String> = vec![];

    let match_combo = |c: i64, cpu: &Computer| match c {
        0..=3 => c,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,
        _ => panic!("{} not found", c),
    };

    loop {
        if pc >= opcodes.len() {
            break;
        }

        let instr = opcodes[pc];
        let combo = opcodes[pc + 1];

        match instr {
            0 => {
                cpu.a = (cpu.a as f64 / 2i64.pow(match_combo(combo, &cpu) as u32) as f64).floor()
                    as i64;
                pc += 2;
            }
            1 => {
                cpu.b = cpu.b ^ combo;
                pc += 2;
            }
            2 => {
                cpu.b = match_combo(combo, &cpu).rem_euclid(8);
                pc += 2;
            }
            3 => {
                if cpu.a == 0 {
                    pc += 2;
                    continue;
                }
                pc = combo as usize;
            }
            4 => {
                cpu.b = cpu.b ^ cpu.c;
                pc += 2;
            }
            5 => {
                out.push(match_combo(combo, &cpu).rem_euclid(8).to_string());
                pc += 2;
            }
            6 => {
                cpu.b = (cpu.a as f64 / 2i64.pow(match_combo(combo, &cpu) as u32) as f64).floor()
                    as i64;
                pc += 2;
            }
            7 => {
                cpu.c = (cpu.a as f64 / 2i64.pow(match_combo(combo, &cpu) as u32) as f64).floor()
                    as i64;
                pc += 2;
            }
            _ => panic!("{} not found", instr),
        }
    }
    out.join(",")
}

pub fn part_one(input: &str) -> Option<String> {
    let mut split = input.split("\n\n");
    let registers: Vec<&str> = split.next().unwrap().split('\n').collect();

    let mut cpu = Computer::default();

    let re = Regex::new(r"^Register (\w): (\d+)$").unwrap();
    for register in registers {
        for cap in re.captures_iter(register) {
            let r: char = cap[1].parse().unwrap();
            let v: i64 = cap[2].parse().unwrap();
            match r {
                'A' => cpu.a = v,
                'B' => cpu.b = v,
                'C' => cpu.c = v,
                _ => panic!("Invalid register"),
            }
        }
    }

    let program: &str = split.next().unwrap();
    let opcodes: Vec<i64> = program
        .split(": ")
        .skip(1)
        .next()
        .map(|r| {
            r.split(',')
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap();

    let out = simulate(&opcodes, &mut cpu);

    Some(out)
}

pub fn part_two(input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
