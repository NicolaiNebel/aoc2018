use std::ops::Range;
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

fn minutes_asleep(v : &[Range<i32>]) -> i32 {
    v.iter().map(|&Range{start,end}| end - start).sum()
}

fn times_asleep(n : &[Range<i32>], minute : i32) -> i32 {
    let mut res = 0;
    for &Range{start : from, end : to} in n {
        if minute >= from && minute < to {
            res += 1;
        }
    }
    res
}

fn minute_most_frequently_asleep(intervals : &[Range<i32>]) -> (i32, i32) {
    let mut minutes : [i32; 60] = [0; 60];
    for &Range{start: from,end: to} in intervals {
        let slice : &mut[i32] = &mut minutes[from as usize .. to as usize];
        for x in slice {
            *x += 1;
        }
    }

    let (x,y) = (0..60).zip(minutes.iter()).max_by_key(|(_x,v)| *v).unwrap();
    (x,*y)
}

fn solve_1(guard_map : &HashMap<i32, Vec<Range<i32>>>) {
    let ret = guard_map
        .iter()
        .max_by_key(|(_k1,v1)| minutes_asleep(v1))
        .unwrap();

    let pick = (0..60).max_by_key(|&x| times_asleep(ret.1, x)).unwrap();
    println!("{}", ret.0 * pick);
}

fn solve_2(guard_map : &HashMap<i32, Vec<Range<i32>>>) {
    let minute_map = guard_map.iter().map(|(&k,v)| (k, minute_most_frequently_asleep(v)));
    let res = minute_map.max_by_key(|(_k,v)| v.1);
    println!("{:?}", res);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sorted_lines: Vec<_> = input.lines().collect();
    sorted_lines.sort_by_key(|&x| x);

    let mut guard_map : HashMap<i32, Vec<Range<i32>>>= HashMap::new();

    let mut current_guard = 0;
    let mut fall_asleep_minute = 0;
    for line in sorted_lines {
        let minute = scan_fmt!(line,
                               "[{*d}-{*d}-{*d} {*d}:{d}]",
                               i32).unwrap();
        let c = line.chars().nth(19).unwrap();
        if c == 'G' {
            let guard = scan_fmt!(line,
                                  "[{*d}-{*d}-{*d} {*d}:{*d}] Guard #{d}",
                                  i32).unwrap();
            current_guard = guard;
            fall_asleep_minute = 0;
        }
        else if c == 'f' {
            fall_asleep_minute = minute;
        }
        else if c == 'w' {
            let vec = guard_map.entry(current_guard).or_insert(Vec::new());
            vec.push(fall_asleep_minute .. minute);
        }
    }
    solve_1(&guard_map);
    solve_2(&guard_map);
}
