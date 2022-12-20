use adventofcode2022::read_input_lines_as;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Num {
    value: i64,
    curr_pos: i64,
    starting_pos: i64,
}

fn do_swaps(mut num: i64, dir: Dir, mover_pos: i64, by_curr_pos: &mut [Rc<RefCell<Num>>]) {
    num %= (by_curr_pos.len() - 1) as i64;
    if num == 0 {
        return;
    }
    let mut other_index = mover_pos
        + match dir {
            Dir::Left => -1,
            Dir::Right => 1,
        };

    if other_index == -1 {
        other_index = by_curr_pos.len() as i64 - 1;
    }
    if other_index == by_curr_pos.len() as i64 {
        other_index = 0;
    }
    by_curr_pos.swap(mover_pos as usize, other_index as usize);
    by_curr_pos[mover_pos as usize].borrow_mut().curr_pos = mover_pos;
    by_curr_pos[other_index as usize].borrow_mut().curr_pos = other_index;
    do_swaps(num - 1, dir, other_index, by_curr_pos);
}

fn mix_one(
    starting_vec_pos: i64,
    by_starting_pos: &[Rc<RefCell<Num>>],
    by_curr_pos: &mut [Rc<RefCell<Num>>],
) {
    let to_move = by_starting_pos[starting_vec_pos as usize].clone();
    let starting_pos = to_move.borrow().curr_pos;
    let value = to_move.borrow().value;
    do_swaps(
        value.abs(),
        if value < 0 { Dir::Left } else { Dir::Right },
        starting_pos,
        by_curr_pos,
    );
}

fn mix(by_starting_pos: &[Rc<RefCell<Num>>], by_curr_pos: &mut [Rc<RefCell<Num>>]) {
    for i in 0..by_starting_pos.len() {
        mix_one(i as i64, by_starting_pos, by_curr_pos);
    }
}

fn find_zero_pos(by_starting_pos: &[Rc<RefCell<Num>>]) -> i64 {
    for item in by_starting_pos {
        if item.borrow().value == 0 {
            return item.borrow().curr_pos;
        }
    }

    panic!("Didn't find 0")
}

enum Dir {
    Left,
    Right,
}

fn part1() {
    let input: Vec<i64> = read_input_lines_as(20);
    //let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
    let items_by_starting_pos: Vec<Rc<RefCell<Num>>> = input
        .into_iter()
        .enumerate()
        .map(|(pos, value)| {
            Rc::new(RefCell::new(Num {
                value,
                curr_pos: pos as i64,
                starting_pos: pos as i64,
            }))
        })
        .collect();
    let mut items_by_curr_pos = items_by_starting_pos.clone();
    mix(&items_by_starting_pos, &mut items_by_curr_pos);
    let idx = find_zero_pos(&items_by_starting_pos);
    let result = items_by_curr_pos[((idx + 1000) % items_by_starting_pos.len() as i64) as usize]
        .borrow()
        .value
        + items_by_curr_pos[((idx + 2000) % items_by_starting_pos.len() as i64) as usize]
            .borrow()
            .value
        + items_by_curr_pos[((idx + 3000) % items_by_starting_pos.len() as i64) as usize]
            .borrow()
            .value;
    println!("Part 1: {result}");
}

const KEY: i64 = 811589153;

fn part2() {
    let input: Vec<i64> = read_input_lines_as(20);
    //let input: Vec<i64> = vec![1, 2, -3, 3, -2, 0, 4];
    let items_by_starting_pos: Vec<Rc<RefCell<Num>>> = input
        .into_iter()
        .enumerate()
        .map(|(pos, value)| {
            Rc::new(RefCell::new(Num {
                value: value * KEY,
                curr_pos: pos as i64,
                starting_pos: pos as i64,
            }))
        })
        .collect();
    let mut items_by_curr_pos = items_by_starting_pos.clone();
    for _ in 0..10 {
        mix(&items_by_starting_pos, &mut items_by_curr_pos);
    }
    let idx = find_zero_pos(&items_by_starting_pos);
    let result = items_by_curr_pos[((idx + 1000) % items_by_starting_pos.len() as i64) as usize]
        .borrow()
        .value
        + items_by_curr_pos[((idx + 2000) % items_by_starting_pos.len() as i64) as usize]
            .borrow()
            .value
        + items_by_curr_pos[((idx + 3000) % items_by_starting_pos.len() as i64) as usize]
            .borrow()
            .value;
    println!("Part 2: {result}");
}

fn main() {
    part1();
    part2();
}
