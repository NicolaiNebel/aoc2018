fn f(mut acc : String, c : char) -> String {
    if acc.len() == 0 {
        acc.push(c);
    } else {
        let prev_char = acc.pop().unwrap();
        
        if !((c.is_lowercase() && c.to_ascii_uppercase() == prev_char) ||
             (c.is_uppercase() && c.to_ascii_lowercase() == prev_char)) {
            acc.push(prev_char);
            acc.push(c);
        }
    }
    acc
}

fn solve_1(input : &String) {
    let res = input
        .chars()
        .fold(String::new(), f)
        .len();
    println!("{}", res);
}

fn solve_2(input : &String) {
    let chars = input.chars();

    let reduced_string = "abdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| chars
                .clone()
                .filter(|r| c.to_ascii_lowercase() != (*r).to_ascii_lowercase())
                .fold(String::new(), f)
                .len())
        .min()
        .unwrap();

    println!("{}", reduced_string - 1);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    solve_1(&input);
    solve_2(&input);

}
