// type Rucksacks = Vec<(&[u8],&[u8])>;

use std::collections::HashSet;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../../inputs/input3.txt");

    let input_numbers = parse_input(contents);
    let input_numbers_2 = parse_input_2(contents);

    let part1 = calc_part1(&input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&input_numbers_2);

    println!("result: {}", part2);

    Ok(())
}

fn parse_input(contents: &str) -> Vec<(&[u8], &[u8])> {
    contents
        .trim()
        .lines()
        .map(|line| line.as_bytes().chunks(line.len() / 2).collect::<Vec<&[u8]>>())
        .map(|chunks| {
            (chunks[0], chunks[1])
        })
        .collect()
}

fn calc_part1(input: &[(&[u8], &[u8])]) -> usize {
    input.iter().map(|rucksack|
        {
            let compartment1: HashSet<u8> = rucksack.0.iter().cloned().collect::<HashSet<u8>>();
            let compartment2 = rucksack.1.iter().cloned().collect::<HashSet<u8>>();
            let mut i = compartment1.intersection(&compartment2);
            *i.next().unwrap()
        })
        .collect::<Vec<u8>>().iter().map(|x| {
        let i1 = *x as usize;

        if i1 < 97 { i1 - 64 + 26 } else { i1 - 96 }
    }).sum()
}

fn parse_input_2(contents: &str) -> Vec<(&[u8], &[u8], &[u8])> {
    contents
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunks| (chunks[0].as_bytes(), chunks[1].as_bytes(), chunks[2].as_bytes()))
        .collect()
}

fn calc_part2(input: &[(&[u8], &[u8], &[u8])]) -> usize {
    input.iter().map(|rucksack|
        {
            let bag1: HashSet<u8> = rucksack.0.iter().cloned().collect::<HashSet<u8>>();
            let bag2 = rucksack.1.iter().cloned().collect::<HashSet<u8>>();
            let bag3 = rucksack.2.iter().cloned().collect::<HashSet<u8>>();
            let set = bag1.intersection(&bag2).cloned().collect::<HashSet<u8>>();
            let mut i = set.intersection(&bag3);
            *i.next().unwrap()
        })
        .collect::<Vec<u8>>().iter().map(|x| {
        let i1 = *x as usize;

        if i1 < 97 { i1 - 64 + 26 } else { i1 - 96 }
    }).sum()
}


#[test]
fn test() {
    let input =
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    let test: Vec<(&[u8], &[u8])> = parse_input(input);
    assert_eq!(calc_part1(&test), 157usize)
}

#[test]
fn test2() {
    let input =
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    let test = parse_input_2(input);
    assert_eq!(calc_part2(&test), 70usize)
}