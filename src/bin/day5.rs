extern crate regex;

use regex::Regex;

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../../inputs/input5.txt");

    let mut input_numbers = parse_input(contents);

    let part1 = calc_part1(&mut input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&mut input_numbers);

    println!("result: {}", part2);

    Ok(())
}

fn parse_input(contents: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let mut split = contents.split("\n\n");

    let stack_indexes = get_stacks(split.next().unwrap());

    let commands = parse_commands(split.next().unwrap());

    (stack_indexes, commands)
}

fn parse_commands(input: &str) -> Vec<Command> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input.lines().map(|line|
        {
            match re.captures(line) {
                Some(cap) => Command {
                    from: cap[2].parse().unwrap(),
                    to: cap[3].parse().unwrap(),
                    amount: cap[1].parse().unwrap(),
                },
                None => panic!("Invalid move string"),
            }
        }
    ).collect()
}

fn get_stacks(input: &str) -> Vec<Vec<char>> {
    let stacks_input = input.lines();
    let highest_stack = stacks_input.clone().count() - 1;
    let stacks_count = stacks_input.clone().last().unwrap().chars().filter(|&x| x != ' ').count();
    let stack_indexes = (1..=stacks_count).map(|x| if x > 1 { (x - 1) * 4 + 1 } else { x }).collect::<Vec<usize>>();

    let input_vec: Vec<&str> = stacks_input.collect();
    // initial list of container stacks
    let mut stacks: Vec<Vec<char>> = (0..stacks_count).map(|_x| vec![]).collect();
    // fill initial stacks
    for line_nr in (0..highest_stack).rev() {
        let cur_line: Vec<char> = input_vec[line_nr].chars().collect();
        stack_indexes.iter().enumerate().for_each(|(i, &index)| {
            if cur_line.len() > index {
                let x1 = cur_line[index];
                if x1 != ' ' {
                    stacks[i].push(x1);
                }
            }
        })
    }
    stacks
}

fn calc_part1(input: &mut (Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut copied = input.0.clone();
    input.1.iter().for_each(|command| {
        for _ in 0..command.amount {
            copied[command.from - 1].pop().map(|val| copied[command.to - 1].push(val));
        }
    });
    String::from_iter(copied.iter().filter_map(|x| x.last()).map(|x| *x).collect::<Vec<char>>())
}

fn calc_part2(input: &mut (Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut copied = input.0.clone();
    input.1.iter().for_each(|command| {
        let rev = (0..command.amount)
            .filter_map(|_| copied[command.from - 1].pop())
            .collect::<Vec<char>>();
        // collect so that the rev command actually does something
        rev.iter()
            .rev()
            .for_each(|x| copied[command.to - 1].push(*x));
    });
    String::from_iter(copied
        .iter()
        .map(|x| x.last().unwrap())
        .map(|x| *x)
        .collect::<Vec<char>>())
}

#[test]
fn test() {
    let input =
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    let mut test = parse_input(&input);
    assert_eq!(calc_part1(&mut test), "CMZ")
}

#[test]
fn test2() {
    let input =
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
move 5 from 3 to 2
move 1 from 2 to 3
";
    let mut test = parse_input(&input);
    assert_eq!(calc_part2(&mut test), "MND")
}

