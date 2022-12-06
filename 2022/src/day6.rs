use std::collections::HashSet;

pub fn solution() {
    let input = include_str!("../input/6.txt")
        .chars()
        .collect::<Vec<char>>();
    // for part 1 change 14 to 4 everywhere
    let mut input_set = HashSet::new();
    for i in 0..input.len() {
        for y in 0..14 {
            input_set.insert(&input[i + y]);
        }
        if input_set.len() == 14 {
            println!("{}", i + 14);
            break;
        }
        input_set.clear();
    }
}
