use adventofcode2022::read_input_lines;

fn read_input() -> Vec<Vec<i32>> {
    let input = read_input_lines(1, true);
    let mut all_elves = vec![];
    let mut curr_elf = vec![];
    for line in input {
        if line.is_empty() {
            if !curr_elf.is_empty() {
                all_elves.push(curr_elf);
                curr_elf = vec![];
            }
        } else {
            let cals: i32 = line.parse().unwrap();
            curr_elf.push(cals);
        }
    }
    if !curr_elf.is_empty() {
        all_elves.push(curr_elf);
    }
    all_elves
}

fn part1() {
    let all_elves = read_input();
    let sums = all_elves.iter().map(|elf| elf.iter().sum::<i32>());
    println!("{}", sums.max().unwrap());
}

fn part2() {
    let mut all_elves = read_input();
    all_elves.sort_by_cached_key(|item| item.iter().sum::<i32>());
    all_elves.reverse();
    let top_three: i32 = all_elves
        .iter()
        .take(3)
        .map(|elf| elf.iter().sum::<i32>())
        .sum();

    println!("{}", top_three);
}

pub fn main() {
    part1();
    part2();
}
