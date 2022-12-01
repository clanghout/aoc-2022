pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("inputs/input1.txt")?;

    let mut input_numbers = parse_input(contents);

    let part1 = calc_part1(&input_numbers);

    println!("result: {}", part1);

    let part2 = calc_part2(&mut input_numbers);

    println!("result: {}", part2);

    Ok(())
}

fn parse_input(contents: String) -> Vec<usize> {
    let res = vec![0];
    contents
        .lines()
        .fold(res, |mut acc, curr| {
            let index = acc.len() - 1;
            match curr.parse::<usize>().ok() {
                Some(x) => acc[index] += x,
                None => acc.push(0),
            };
            acc
        },
        )
}

fn calc_part1(input: &[usize]) -> &usize {
    input.iter().max().unwrap()
}

fn calc_part2(input: &mut [usize]) -> usize {
    input.sort_unstable_by(|a, b| b.cmp(a));
    input[0] + input[1] + input[2]
}


#[test]
fn test() {
    let input = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");
    let test: Vec<usize> = parse_input(input);
    assert_eq!(*calc_part1(&test), 24000usize)
}

#[test]
fn test2() {
    let input = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");
    let mut test: Vec<usize> = parse_input(input);
    assert_eq!(calc_part2(&mut test), 45000usize)
}