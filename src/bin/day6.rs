use std::collections::HashSet;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = include_str!("../../inputs/input6.txt");

    let input_numbers = parse_input(contents);

    let part1 = calc_part1(&input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&    input_numbers);

    println!("result: {}", part2);

    Ok(())
}

fn parse_input(contents: &str) -> Vec<char> {
    contents
        .trim()
        .chars()
        .collect()
}

fn calc_part1(input: &[char]) -> usize {
    input
        .windows(4)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<&char>>().len() == 4)
        .unwrap().0+4
}

fn calc_part2(input: &[char]) -> usize {
    input
        .windows(14)
        .enumerate()
        .find(|(_, window)| window.iter().collect::<HashSet<&char>>().len() == 14)
        .unwrap().0+14

}

#[test]
fn test() {
    let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb
    ";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 7usize)
}
#[test]
fn test1_2() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz
    ";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 5usize)
}
#[test]
fn test1_3() {
    let input = "nppdvjthqldpwncqszvftbrmjlhg
    ";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 6usize)
}
#[test]
fn test1_4() {
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
    ";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 10usize)
}
#[test]
fn test1_5() {
    let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
    ";
    let test = parse_input(&input);
    assert_eq!(calc_part1(&test), 11usize)
}

#[test]
fn test2_1(){
let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let test = parse_input(&input);
    assert_eq!(calc_part2(&test), 19usize)
}