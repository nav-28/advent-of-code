fn check_range(i1: i32, i2: i32, i3: i32, i4: i32) -> bool {
    if i3 >= i1 && i4 <= i2 {
        return true;
    } else if i1 >= i3 && i2 <= i4 {
        return true;
    } else {
        return false;
    }
}

fn check_overlap(i1: i32, i2: i32, i3: i32, i4: i32) -> bool {
    if (i3 >= i2 && i3 <= i1) || (i1 >= i3 && i1 <= i4) {
        return true;
    } else if (i2 <= i4 && i2 >= i3) || (i4 <= i2 && i4 >= i1) {
        return true;
    } else {
        return false;
    }
}

fn parse_line(line: &str) -> Option<(i32, i32, i32, i32)> {
    let mut iter = line.split([',', '-']);
    Some((
        iter.next()?.parse().ok()?,
        iter.next()?.parse().ok()?,
        iter.next()?.parse().ok()?,
        iter.next()?.parse().ok()?,
    ))
}

pub fn solution() {
    let mut input = include_str!("../input/4.txt")
        .split("\n")
        .collect::<Vec<&str>>();
    input.pop().unwrap(); // remove extra line
    let sum = input
        .iter()
        .map(|line| parse_line(&line).unwrap())
        .filter(|(a, b, x, y)| check_range(*a, *b, *x, *y))
        .count();
    let sum2 = input
        .iter()
        .map(|line| parse_line(&line).unwrap())
        .filter(|(a, b, x, y)| check_overlap(*a, *b, *x, *y))
        .count();
    println!("{sum} {sum2}");
}
