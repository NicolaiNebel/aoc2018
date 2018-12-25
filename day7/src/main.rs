use std::collections::HashSet;

#[macro_use] extern crate scan_fmt;

struct Edge {
    from : char,
    to   : char,
}

fn can_do(c : char, result : &String, edge_list : &Vec<Edge>) -> bool {
    if result.contains(c) {
        return false;
    }

    for edge in edge_list {
        if edge.to == c && !result.contains(edge.from) {
            return false;
        }
    }
    true
}

fn solve_1(chars : &HashSet<char>, edge_list : &Vec<Edge>) {
    let mut result : String = String::new();

    let chars_iter = chars.iter();

    loop {
        // Find todo set
        let mut todo : Vec<char> = chars_iter
            .clone()
            .map(|x| *x)
            .filter(|c| can_do(*c, &result, &edge_list))
            .collect();
        // Test whether it is empty
        if todo.len() == 0 {
            break;
        }
        // Do a thing
        todo.sort();
        result.push(*todo.get(0).unwrap());
    }

    println!("{}", result);
}

fn work(c : char) -> i32 {
    match c {
        'A' => 61,
        'B' => 62,
        'C' => 63,
        'D' => 64,
        'E' => 65,
        'F' => 66,
        'G' => 67,
        'H' => 68,
        'I' => 69,
        'J' => 70,
        'K' => 71,
        'L' => 72,
        'M' => 73,
        'N' => 74,
        'O' => 75,
        'P' => 76,
        'Q' => 77,
        'R' => 78,
        'S' => 79,
        'T' => 80,
        'U' => 81,
        'V' => 82,
        'W' => 83,
        'X' => 84,
        'Y' => 85,
        'Z' => 86,
        _   => panic!("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    }
}


fn solve_2(chars : &HashSet<char>, edge_list : &Vec<Edge>) {
    let mut result = String::new();
    let mut time = 0;
    let mut working : Vec<(char, i32)> = Vec::new();

    let chars_iter = chars.iter();

    loop {
        let doing : HashSet<char> = working.iter().map(|(c, _i)| *c).collect();

        let mut todo : Vec<char> = chars_iter
            .clone()
            .map(|x| *x)
            .filter(|c| can_do(*c, &result, &edge_list))
            .filter(|c| !doing.contains(c))
            .collect();
        todo.sort();

        if working.len() == 0 && todo.len() == 0 {
            break
        }

        for &c in todo.iter().take(5 - working.len()) {
            working.push((c, work(c)));
        }

        // Take list of workers, and figure out time to next task being done
        // Pop that task, subtract that amount from all tasks,
        // return new task list and finished task
        working.sort_by_key(|(_c, i)| -i);
        let (c, step) = working.pop().unwrap();
        working = working.iter().map(|(c,i)| (*c, i - step)).collect();

        result.push(c);
        time += step;
    }
    println!("{} {}", result, time);

}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut edge_list : Vec<Edge> = Vec::new();
    let mut chars : HashSet<char> = HashSet::new();

    for line in input.lines() {
        let (from, to) = scan_fmt!(line,
                                   "Step {[A-Z]} must be finished before step {[A-Z]} can begin.",
                                   char, char);
        let (from, to) = (from.unwrap(), to.unwrap());
        edge_list.push(Edge{ from, to });
        chars.insert(from);
        chars.insert(to);
    }

    solve_1(&chars, &edge_list);
    solve_2(&chars, &edge_list);
}
