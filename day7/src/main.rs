use std::collections::HashSet;

#[macro_use] extern crate scan_fmt;

struct Edge {
    from : char,
    to   : char,
}

fn can_do(c : char, result : &String, edge_list : &Vec<Edge>) -> bool {
       !result.contains(c)
    && !edge_list.iter().any(|edge| edge.to == c && !result.contains(edge.from))
}

fn solve_1(chars : &HashSet<char>, edge_list : &Vec<Edge>) {
    let mut result : String = String::new();

    loop {
        // Find todo set
        let todo = chars
            .iter()
            .cloned()
            .filter(|&c| can_do(c, &result, &edge_list));

        if let Some(value) = todo.min() {
            result.push(value)
        } else {
            break
        }
    }

    println!("{}", result);
}

fn work(c : char) -> i32 {
    (c as i32) - ('A' as i32) + 61
}


fn solve_2(chars : &HashSet<char>, edge_list : &Vec<Edge>) {
    let mut result = String::new();
    let mut time = 0;
    let mut working : Vec<(char, i32)> = Vec::new();

    loop {
        let doing : HashSet<char> = working.iter().map(|(c, _i)| *c).collect();

        let mut todo : Vec<char> = chars
            .iter()
            .cloned()
            .filter(|c| can_do(*c, &result, &edge_list))
            .filter(|c| !doing.contains(c))
            .collect();
        todo.sort();

        if working.is_empty() && todo.is_empty() {
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
        working = working.iter().cloned().map(|(c,i)| (c, i - step)).collect();

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
